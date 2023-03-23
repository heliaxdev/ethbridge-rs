mod templates;

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use clap::Parser;
use ethers_contract::Abigen;
use eyre::{eyre as err, WrapErr};
use itertools::Itertools;
use proc_macro2::{Group, Span, TokenStream, TokenTree};
use quote::{quote, ToTokens};

use self::templates::CargoTomlDepMeta;

const ETHABI_VERSION: &str = "18.0.0";
const ETHERS_VERSION: &str = "2.0.0";
const SMALLVEC_VERSION: &str = "1.10.0";

const FEATURE_GATE_ETHERS: &str = "ethers-derive";

struct Paths {
    /// Path to the output directory of the generated crates.
    output_dir: PathBuf,
    /// Path to the ABI files directory.
    abi_files_dir: PathBuf,
}

#[derive(Debug, Default)]
struct AllEvents {
    bridge: BTreeSet<syn::Ident>,
    governance: BTreeSet<syn::Ident>,
}

/// Generate Ethereum bridge Rust types compatible with Namada's
/// Rust code
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to the directory to parse ABI files from
    #[arg(short = 'p', long, default_value_t = String::from("target"))]
    abi_files_dir: String,

    /// The git tag of `ethereum-bridge` whose artifacts
    /// we are to download. If no tag is provided, we try
    /// to use files present in the specified ABI files
    /// directory
    #[arg(short = 't', long)]
    ethereum_bridge_tag: Option<String>,

    /// Path to the output directory of the generated crates.
    /// If no output directory is specified, the current working
    /// directory is used instead
    #[arg(short = 'o', long)]
    output_dir: Option<String>,

    /// The version of the generated crates. If not specified, the
    /// version of this CLI command is used instead
    #[arg(short = 'v', long)]
    crate_version: Option<String>,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
    }
}

fn run() -> eyre::Result<()> {
    let Args {
        output_dir,
        abi_files_dir,
        ethereum_bridge_tag,
        crate_version,
    } = Args::parse();

    let crate_version = crate_version.unwrap_or_else(|| env!("CARGO_PKG_VERSION").into());
    let paths = Paths {
        output_dir: output_dir.map(|s| s.into()).unwrap_or_else(PathBuf::new),
        abi_files_dir: abi_files_dir.into(),
    };
    if let Some(tag) = ethereum_bridge_tag {
        download_abi_files(tag, &paths)?;
    }

    let mut structs = BTreeMap::new();
    let mut events = AllEvents::default();

    generate_crates("Bridge", &crate_version, &paths, &mut structs, &mut events)?;
    generate_crates(
        "Governance",
        &crate_version,
        &paths,
        &mut structs,
        &mut events,
    )?;

    generate_structs_crate(&paths, &crate_version, structs)?;
    generate_events_crate(&paths, &crate_version, events)?;

    Ok(())
}

fn generate_structs_crate(
    paths: &Paths,
    crate_version: &str,
    mut structs: BTreeMap<String, syn::ItemStruct>,
) -> eyre::Result<()> {
    process_structs(&mut structs);
    generate_crate_template(
        "ethbridge-structs".into(),
        crate_version,
        [
            (
                "ethabi".into(),
                CargoTomlDepMeta {
                    version: ETHABI_VERSION.into(),
                    optional: false,
                    ..Default::default()
                },
            ),
            (
                "ethers-contract".into(),
                CargoTomlDepMeta {
                    version: ETHERS_VERSION.into(),
                    optional: true,
                    ..Default::default()
                },
            ),
            (
                "ethers".into(),
                CargoTomlDepMeta {
                    version: ETHERS_VERSION.into(),
                    optional: true,
                    ..Default::default()
                },
            ),
        ],
        [(
            FEATURE_GATE_ETHERS.into(),
            vec!["ethers".into(), "ethers-contract".into()],
        )],
        paths,
    )?;
    generate_crate_source(
        "ethbridge-structs".into(),
        paths,
        std::iter::once(quote! {
            #![allow(dead_code)]
        })
        .chain(structs.into_values().map(|s| s.into_token_stream())),
    )?;
    Ok(())
}

fn generate_events_crate(
    paths: &Paths,
    crate_version: &str,
    events: AllEvents,
) -> eyre::Result<()> {
    generate_crate_template(
        "ethbridge-events".into(),
        crate_version,
        [
            (
                "smallvec".into(),
                CargoTomlDepMeta {
                    version: SMALLVEC_VERSION.into(),
                    optional: false,
                    ..Default::default()
                },
            ),
            (
                "ethers".into(),
                CargoTomlDepMeta {
                    version: ETHERS_VERSION.into(),
                    optional: false,
                    ..Default::default()
                },
            ),
            (
                "ethbridge-bridge-events".into(),
                CargoTomlDepMeta {
                    version: String::new(),
                    optional: false,
                    feats: vec!["ethers-derive".into()],
                },
            ),
            (
                "ethbridge-governance-events".into(),
                CargoTomlDepMeta {
                    version: String::new(),
                    optional: false,
                    feats: vec!["ethers-derive".into()],
                },
            ),
        ],
        [],
        paths,
    )?;
    let all_dyn_events = events
        .bridge
        .iter()
        .chain(events.governance.iter())
        .map(|event_name| {
            quote! {
                &::std::marker::PhantomData::<#event_name>,
            }
        })
        .fold(TokenStream::new(), |mut stream, other| {
            stream.extend(other);
            stream
        });
    let event_codec_impls = [("Bridge", events.bridge), ("Governance", events.governance)]
        .into_iter()
        .map(|(kind, events)| {
            let kind_events =
                syn::Ident::new(&format!("{kind}Events"), Span::call_site()).to_token_stream();
            let kind = syn::Ident::new(kind, Span::call_site()).to_token_stream();
            events
                .into_iter()
                .fold(TokenStream::new(), |mut stream, event_ident| {
                    stream.extend(quote! {
                        impl EventCodec for ::std::marker::PhantomData<#event_ident> {
                            fn event_signature(&self) -> ::std::borrow::Cow<'static, str> {
                                #event_ident :: abi_signature()
                            }

                            fn kind(&self) -> EventKind {
                                EventKind :: #kind
                            }

                            fn decode(
                                &self,
                                log: &::ethers::abi::RawLog,
                            ) -> Result<Events, AbiError> {
                                let encoded_event = {
                                    let buf_len = if !log.topics.is_empty() {
                                        log.data.len() + (log.topics.len() - 1) * 32
                                    } else {
                                        log.data.len()
                                    };
                                    let mut buf: SmallVec<[u8; 1024]> =
                                        smallvec![0; buf_len];
                                    let mut ptr = 0;
                                    for topic in log.topics.iter().skip(1) {
                                        let end = ptr + 32;
                                        buf[ptr..end].copy_from_slice(&topic.0[..]);
                                        ptr = end;
                                    }
                                    buf[ptr..].copy_from_slice(&log.data[..]);
                                    buf
                                };
                                let event = #event_ident :: decode(encoded_event)?;
                                Ok(Events :: #kind (
                                    #kind_events :: #event_ident ( event )
                                ))
                            }
                        }
                    });
                    stream
                })
        })
        .fold(TokenStream::new(), |mut stream, other| {
            stream.extend(other);
            stream
        });
    generate_crate_source(
        "ethbridge-events".into(),
        paths,
        std::iter::once(quote! {
            #![allow(dead_code)]
            #![allow(unused_imports)]

            use ::ethbridge_bridge_events::*;
            use ::ethbridge_governance_events::*;
            use ::ethers::contract::{AbiError, EthEvent};
            use ::ethers::abi::AbiDecode;
            use ::smallvec::{smallvec, SmallVec};

            ///Codec to deserialize Ethereum events.
            pub trait EventCodec {
                ///ABI signature of the Ethereum event.
                fn event_signature(&self) -> ::std::borrow::Cow<'static, str>;

                ///The kind of event (Bridge or Governance).
                fn kind(&self) -> EventKind;

                ///Decode an Ethereum event.
                fn decode(
                    &self,
                    log: &::ethers::abi::RawLog,
                ) -> Result<Events, AbiError>;
            }

            #event_codec_impls

            ///Return all Ethereum event codecs.
            pub fn event_codecs() -> &'static [&'static dyn EventCodec] {
                &[#all_dyn_events]
            }

            ///The Ethereum events generated by `ethbridge-rs`.
            #[derive(Debug)]
            pub enum Events {
                /// Bridge events.
                Bridge(BridgeEvents),
                /// Governance events.
                Governance(GovernanceEvents),
            }

            ///The kinds of Ethereum events generated by `ethbridge-rs`.
            #[derive(Debug)]
            pub enum EventKind {
                /// Bridge events.
                Bridge,
                /// Governance events.
                Governance,
            }
        }),
    )?;
    Ok(())
}

fn process_events(events: TokenStream, all_events: &mut BTreeSet<syn::Ident>) -> TokenStream {
    let mut events_file =
        syn::parse2::<syn::File>(events).expect("The generated code is syntactically correct");
    events_file.items.iter_mut().for_each(|item| match item {
        impl_ @ syn::Item::Impl(_) => process_events_impl(impl_),
        enum_ @ syn::Item::Enum(_) => process_events_enum(enum_),
        struct_ @ syn::Item::Struct(_) => process_events_struct(struct_, all_events),
        _ => (),
    });
    events_file.into_token_stream()
}

fn add_toks_before_item<I: ToTokens + syn::parse::Parse>(item: &mut I, toks_before: TokenStream) {
    let item_tokens = item.to_token_stream();
    *item = syn::parse2(quote! {
        #toks_before
        #item_tokens
    })
    .expect("Should have the right syntax");
}

fn process_events_impl(item: &mut syn::Item) {
    let should_feature_gate = {
        let syn::Item::Impl(impl_) = item else {
            unreachable!()
        };
        impl_
            .trait_
            .as_ref()
            .map(|(_, path, _)| {
                let path = path.to_token_stream().to_string();
                path.contains("ethers_contract") || path.contains("Display")
            })
            .unwrap_or(false)
    };

    // nothing to do
    if !should_feature_gate {
        return;
    }

    let feature_gate = FEATURE_GATE_ETHERS.to_token_stream();
    add_toks_before_item(
        item,
        quote! {
            #[cfg(feature = #feature_gate)]
        },
    );
}

fn process_events_enum(item: &mut syn::Item) {
    if let syn::Item::Enum(enum_) = item {
        let derives = enum_.attrs[1]
            .clone()
            .tokens
            .into_iter()
            .map(|derives_group| {
                if let TokenTree::Group(g) = derives_group {
                    g
                } else {
                    unreachable!()
                }
            })
            .next()
            .expect("Should have derives");

        let (derives, cfgs) = process_derives_tt(derives);
        enum_.attrs[1].tokens = quote! {
            (#derives)
        };

        let _ = enum_;
        add_toks_before_item(item, cfgs);
    }
}

fn process_events_struct(item: &mut syn::Item, all_events: &mut BTreeSet<syn::Ident>) {
    if let syn::Item::Struct(struct_) = item {
        all_events.insert(struct_.ident.clone());

        let ethevent_before_struct = struct_.attrs.remove(1).tokens;

        let derives = struct_.attrs[0]
            .clone()
            .tokens
            .into_iter()
            .map(|derives_group| {
                if let TokenTree::Group(g) = derives_group {
                    g
                } else {
                    unreachable!()
                }
            })
            .next()
            .expect("Should have derives");

        let (derives, cfgs) = process_derives_tt(derives);
        struct_.attrs[0].tokens = quote! {
            (#derives)
        };

        if let syn::Fields::Named(fields) = &mut struct_.fields {
            process_fields(fields);
        }

        let _ = struct_;
        let feature_gate = FEATURE_GATE_ETHERS.to_token_stream();

        add_toks_before_item(
            item,
            quote! {
                #cfgs
                #[cfg_attr(feature = #feature_gate, ethevent #ethevent_before_struct)]
                #[cfg_attr(
                    feature = #feature_gate,
                    derive(::ethers_contract::EthAbiCodec)
                )]
            },
        );
    }
}

fn process_derives_tt(derives: Group) -> (TokenStream, TokenStream) {
    let mut derives = derives
        .stream()
        .into_iter()
        .group_by(|tt| {
            if let TokenTree::Punct(p) = tt {
                p.as_char() == ','
            } else {
                false
            }
        })
        .into_iter()
        .filter_map(|(is_comma, g)| (!is_comma).then_some(g.collect_vec()))
        .collect_vec();
    let mut ethers_derives = Vec::new();
    for derive in derives.iter_mut() {
        if derive.len() == 1 {
            continue;
        }
        if derive[0].to_string() != ":" {
            continue;
        }
        if derive[1].to_string() != ":" {
            continue;
        }
        if derive[2].to_string() != "ethers_contract" {
            continue;
        }
        let derive = std::mem::take(derive);
        ethers_derives.push(derive);
    }
    let derives = derives
        .into_iter()
        .filter_map(|tts| {
            if tts.is_empty() {
                return None;
            }
            let tt = tts.into_iter().map_into::<TokenStream>().fold(
                TokenStream::new(),
                |mut stream, other| {
                    stream.extend(other);
                    stream
                },
            );
            Some(quote! { #tt, })
        })
        .fold(TokenStream::new(), |mut stream, other| {
            stream.extend(other);
            stream
        });
    let feature_gate = FEATURE_GATE_ETHERS.to_token_stream();
    let cfgs = ethers_derives
        .into_iter()
        .map(|tts| {
            let mut stream = TokenStream::new();
            stream.extend(tts);
            stream
        })
        .fold(TokenStream::new(), |mut stream, derive| {
            stream.extend(quote! {
                #[cfg_attr(feature = #feature_gate, derive(#derive))]
            });
            stream
        });
    (derives, cfgs)
}

fn process_fields(fields: &mut syn::FieldsNamed) {
    for field in fields.named.iter_mut() {
        // feature gate attributes
        if !field.attrs.is_empty() {
            let mut ethevent_attr = field.attrs.remove(0);

            let feature_gate = FEATURE_GATE_ETHERS.to_token_stream();
            let ethevent = ethevent_attr.tokens;
            ethevent_attr.tokens = quote! {
                cfg_attr(feature = #feature_gate, ethevent #ethevent)
            };
            ethevent_attr.path.segments.clear();

            field.attrs.insert(0, ethevent_attr);
        }

        // use `ethabi` instead of `ethers`
        let mut ty = field.ty.to_token_stream().into_iter().collect_vec();

        let field_type_prefix = ty[0].to_string();
        match field_type_prefix.as_str() {
            ":" if ty.len() >= 3 && ty[2].to_string() == "ethers" => {
                let imported_type = ty.pop().unwrap();
                ty.clear();
                ty.extend(
                    quote! {
                        ::ethabi::ethereum_types::#imported_type
                    }
                    .into_iter(),
                );
            }
            // NOTE: handle edge cases here. hashmaps and such
            // may be missing, if they're generated, later, on
            // smart contract revisions
            ":" if ty.len() >= 11 && ty[8].to_string() == "Vec" && ty[10].to_string() == ":" => {
                ty.pop().unwrap(); // pop last bracket
                let imported_type = ty.pop().unwrap();
                ty.truncate(9);
                ty.extend(quote! {
                    <::ethabi::ethereum_types::#imported_type>
                });
            }
            _ => {}
        }

        field.ty = syn::parse2(ty.into_iter().fold(TokenStream::new(), |mut stream, tt| {
            stream.extend(std::iter::once(tt));
            stream
        }))
        .expect("Should have the right syntax");
    }
}

fn process_structs(structs: &mut BTreeMap<String, syn::ItemStruct>) {
    for s in structs.values_mut() {
        // process derives - make ethers derives optional
        let derives = s.attrs[1]
            .clone()
            .tokens
            .into_iter()
            .map(|derives_group| {
                if let TokenTree::Group(g) = derives_group {
                    g
                } else {
                    unreachable!()
                }
            })
            .next()
            .expect("Should have derives");
        let (derives, cfgs) = process_derives_tt(derives);
        s.attrs[1].tokens = quote! {
            (#derives)
        };
        add_toks_before_item(s, cfgs);
        // process fields - should use types from `ethabi` instead
        if let syn::Fields::Named(fields) = &mut s.fields {
            process_fields(fields);
        }
    }
}

fn generate_crates(
    abi_file: &str,
    version: &str,
    paths: &Paths,
    structs: &mut BTreeMap<String, syn::ItemStruct>,
    events: &mut AllEvents,
) -> eyre::Result<()> {
    let abi_file_path = paths.abi_files_dir.join(format!("{abi_file}.abi"));
    let abi_gen = Abigen::from_file(&abi_file_path)
        .with_context(|| format!("file not found: {}", abi_file_path.display()))?;
    let (abi, _) = abi_gen.expand()?;
    let mut structs_iter = abi.abi_structs.into_iter();
    loop {
        let Some(tt) = structs_iter.next() else {
            break;
        };
        let mut tts: Vec<TokenStream> = vec![tt.into()];
        for _ in 0..5 {
            tts.push(
                structs_iter
                    .next()
                    .ok_or_else(|| err!("insufficient token trees in generated rust code"))?
                    .into(),
            );
        }
        let Some(TokenTree::Ident(ident)) = structs_iter.next() else {
            eyre::bail!("expected identifier in generated rust code, but got something else");
        };
        let key = ident.to_string();
        tts.push(TokenTree::Ident(ident).into());
        tts.push(
            structs_iter
                .next()
                .ok_or_else(|| err!("struct definition not found in generated rust code"))?
                .into(),
        );
        let struct_stream = tts
            .into_iter()
            .fold(TokenStream::new(), |mut stream, other| {
                stream.extend(other.into_iter().map(TokenStream::from));
                stream
            });
        let s: syn::ItemStruct =
            syn::parse2(struct_stream).context("invalid struct has been parsed")?;
        structs.insert(key, s);
    }
    generate_crate_template(
        get_subcrate(abi_file, "calls"),
        version,
        [
            (
                "ethbridge-structs".into(),
                CargoTomlDepMeta {
                    version: String::new(),
                    optional: false,
                    feats: vec!["ethers-derive".into()],
                },
            ),
            (
                "ethers".into(),
                CargoTomlDepMeta {
                    version: ETHERS_VERSION.into(),
                    optional: false,
                    ..Default::default()
                },
            ),
            (
                "ethers-contract".into(),
                CargoTomlDepMeta {
                    version: ETHERS_VERSION.into(),
                    optional: false,
                    ..Default::default()
                },
            ),
        ],
        [],
        paths,
    )?;
    generate_crate_source(
        get_subcrate(abi_file, "calls"),
        paths,
        std::iter::once(quote! {
            #![allow(dead_code)]
            use ::ethbridge_structs::*;
        })
        .chain(abi.call_structs.into_iter().map(|tt| tt.into())),
    )?;
    generate_crate_template(
        get_subcrate(abi_file, "events"),
        version,
        [
            (
                "ethbridge-structs".into(),
                CargoTomlDepMeta {
                    version: String::new(),
                    optional: false,
                    ..Default::default()
                },
            ),
            (
                "ethers".into(),
                CargoTomlDepMeta {
                    version: ETHERS_VERSION.into(),
                    optional: true,
                    ..Default::default()
                },
            ),
            (
                "ethers-contract".into(),
                CargoTomlDepMeta {
                    version: ETHERS_VERSION.into(),
                    optional: true,
                    ..Default::default()
                },
            ),
            (
                "ethabi".into(),
                CargoTomlDepMeta {
                    version: ETHABI_VERSION.into(),
                    optional: false,
                    ..Default::default()
                },
            ),
        ],
        [(
            FEATURE_GATE_ETHERS.into(),
            vec![
                "ethers".into(),
                "ethers-contract".into(),
                "ethbridge-structs/ethers-derive".into(),
            ],
        )],
        paths,
    )?;
    let mut all_events = BTreeSet::new();
    generate_crate_source(
        get_subcrate(abi_file, "events"),
        paths,
        std::iter::once(quote! {
            #![allow(dead_code)]
            #![allow(unused_imports)]
            use ::ethbridge_structs::*;
        })
        .chain(
            process_events(abi.events, &mut all_events)
                .into_iter()
                .map(|tt| tt.into()),
        ),
    )?;
    dispatch_on_abi(
        abi_file,
        || {
            events.bridge.extend(all_events.iter().cloned());
        },
        || {
            events.governance.extend(all_events.iter().cloned());
        },
    );
    generate_crate_template(
        get_subcrate(abi_file, "contract"),
        version,
        [
            dispatch_on_abi(
                abi_file,
                || {
                    (
                        "ethbridge-bridge-events".into(),
                        CargoTomlDepMeta {
                            version: String::new(),
                            optional: false,
                            feats: vec!["ethers-derive".into()],
                        },
                    )
                },
                || {
                    (
                        "ethbridge-governance-events".into(),
                        CargoTomlDepMeta {
                            version: String::new(),
                            optional: false,
                            feats: vec!["ethers-derive".into()],
                        },
                    )
                },
            ),
            (
                "ethbridge-structs".into(),
                CargoTomlDepMeta {
                    version: String::new(),
                    optional: false,
                    ..Default::default()
                },
            ),
            (
                "ethers".into(),
                CargoTomlDepMeta {
                    version: ETHERS_VERSION.into(),
                    optional: false,
                    ..Default::default()
                },
            ),
            (
                "ethers-contract".into(),
                CargoTomlDepMeta {
                    version: ETHERS_VERSION.into(),
                    optional: false,
                    ..Default::default()
                },
            ),
        ],
        [],
        paths,
    )?;
    generate_crate_source(
        get_subcrate(abi_file, "contract"),
        paths,
        std::iter::once(quote! {
            #![allow(dead_code)]
            #![allow(unused_imports)]
            #![allow(clippy::too_many_arguments)]
            use ::ethbridge_structs::*;
        })
        .chain(
            dispatch_on_abi(
                abi_file,
                || {
                    quote! {
                        use ::ethbridge_bridge_events::*;
                    }
                },
                || {
                    quote! {
                        use ::ethbridge_governance_events::*;
                    }
                },
            )
            .into_iter()
            .map(|tt| tt.into()),
        )
        .chain(abi.contract.into_iter().map(|tt| tt.into())),
    )?;
    Ok(())
}

fn dispatch_on_abi<T, F1, F2>(abi_file: &str, mut bridge: F1, mut governance: F2) -> T
where
    F1: FnMut() -> T,
    F2: FnMut() -> T,
{
    match abi_file {
        "Bridge" => bridge(),
        "Governance" => governance(),
        _ => unreachable!("unknown ABI file type"),
    }
}

fn generate_crate_template(
    crate_name: String,
    crate_version: &str,
    deps: impl Into<BTreeMap<String, CargoTomlDepMeta>>,
    feats: impl Into<BTreeMap<String, Vec<String>>>,
    paths: &Paths,
) -> eyre::Result<()> {
    let crate_path = paths.output_dir.join(&crate_name);
    let cargo_toml_path = crate_path.join("Cargo.toml");
    std::fs::create_dir_all(crate_path.join("src"))
        .with_context(|| format!("failed to create directory: {}", crate_path.display()))?;
    let cargo_template = templates::cargo();
    let mut tera_context = tera::Context::new();
    tera_context.insert("crate_name", &crate_name);
    tera_context.insert("crate_version", &crate_version);
    tera_context.insert("dependencies", &deps.into());
    tera_context.insert("features", &feats.into());
    tera_context.insert("feature_gate_ethers", FEATURE_GATE_ETHERS);
    let pretty_toml = {
        let toml_str = cargo_template
            .render("Cargo.toml", &tera_context)
            .context("failed to render Cargo.toml")?;
        let table: toml::map::Map<String, toml::Value> =
            toml::from_str(&toml_str).context("failed to deserialize toml")?;
        toml::to_string_pretty(&table).context("failed to serialize toml")?
    };
    let err = std::fs::write(&cargo_toml_path, pretty_toml);
    err.with_context(|| format!("failed to create file: {}", cargo_toml_path.display()))
}

fn generate_crate_source(
    crate_name: String,
    paths: &Paths,
    source: impl IntoIterator<Item = TokenStream>,
) -> eyre::Result<()> {
    let lib_path = paths.output_dir.join(crate_name).join("src").join("lib.rs");
    let source = source
        .into_iter()
        .fold(TokenStream::new(), |mut stream, other| {
            stream.extend(other);
            stream
        })
        .to_string();
    std::fs::write(&lib_path, source)
        .with_context(|| format!("failed to create file: {}", lib_path.display()))
}

fn get_subcrate(abi_file: &str, suffix: &str) -> String {
    format!("ethbridge-{}-{suffix}", abi_file.to_lowercase())
}

fn download_abi_files(_tag: String, _paths: &Paths) -> eyre::Result<()> {
    eyre::bail!("downloading of ABI artifacts is not implemented yet")
}
