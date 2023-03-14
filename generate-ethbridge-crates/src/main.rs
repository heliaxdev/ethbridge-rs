mod templates;

use std::collections::BTreeMap;
use std::path::PathBuf;

use clap::Parser;
use ethers_contract::Abigen;
use eyre::{eyre as err, WrapErr};
use itertools::Itertools;
use proc_macro2::{TokenStream, TokenTree};
use quote::{quote, ToTokens};

use self::templates::CargoTomlDepMeta;

const ETHABI_VERSION: &str = "18.0.0";
const ETHERS_VERSION: &str = "2.0.0";

const FEATURE_GATE_ETHERS: &str = "ethers-derive";

struct Paths {
    /// Path to the output directory of the generated crates.
    output_dir: PathBuf,
    /// Path to the ABI files directory.
    abi_files_dir: PathBuf,
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

    generate_crates("Bridge", &crate_version, &paths, &mut structs)?;
    generate_crates("Governance", &crate_version, &paths, &mut structs)?;

    process_structs(&mut structs);
    generate_crate_template(
        "ethbridge-structs".into(),
        &crate_version,
        [
            (
                "ethabi".into(),
                CargoTomlDepMeta {
                    version: ETHABI_VERSION.into(),
                    optional: false,
                },
            ),
            (
                "ethers".into(),
                CargoTomlDepMeta {
                    version: ETHERS_VERSION.into(),
                    optional: true,
                },
            ),
        ],
        [(FEATURE_GATE_ETHERS.into(), vec!["ethers".into()])],
        &paths,
    )?;
    //println!("{:#?}", structs.values().collect::<Vec<_>>());
    generate_crate_source(
        "ethbridge-structs".into(),
        &paths,
        std::iter::once(quote! {
            #![allow(dead_code)]
        })
        .chain(structs.into_values().flatten()),
    )?;

    Ok(())
}

fn process_structs(structs: &mut BTreeMap<String, Vec<TokenStream>>) {
    for s in structs.values_mut() {
        // process derives - make ethers derives optional
        let TokenTree::Group(derives) = s[3].clone().into_iter().next().unwrap() else {
            panic!("should have derives");
        };
        let mut derives = derives.stream().into_iter();
        derives.next(); // skip first tok "derive"
        let TokenTree::Group(derives) = derives.next().unwrap() else {
            panic!("should have derives");
        };
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
            derive.clear();
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
        s[3] = quote! {
            [derive(
                #derives
            )]
            #[cfg_attr(feature = #feature_gate, derive(::ethers::contract::EthAbiType))]
            #[cfg_attr(feature = #feature_gate, derive(::ethers::contract::EthAbiCodec))]
        };
        // process fields - should use types from `ethabi` instead
        let TokenTree::Group(fields) = s[7].clone().into_iter().next().unwrap() else {
            panic!("should have derives");
        };
        let mut fields = fields
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
        for field in fields.iter_mut() {
            let field_type_prefix = field[3].to_string();
            match field_type_prefix.as_str() {
                ":" if field.len() >= 6 && field[5].to_string() == "ethers" => {
                    let imported_type = field.pop().unwrap();
                    field.truncate(3);
                    field.extend(
                        quote! {
                            ::ethabi::ethereum_types::#imported_type
                        }
                        .into_iter(),
                    );
                }
                // NOTE: handle edge cases here. hashmaps and such
                // may be missing, if they're generated, later, on
                // smart contract revisions
                ":" if field.len() >= 14
                    && field[11].to_string() == "Vec"
                    && field[13].to_string() == ":" =>
                {
                    field.pop().unwrap(); // pop last bracket
                    let imported_type = field.pop().unwrap();
                    field.truncate(12);
                    field.extend(quote! {
                        <::ethabi::ethereum_types::#imported_type>
                    });
                }
                _ => {}
            }
        }
        let fields = fields
            .into_iter()
            .map(|tts| {
                let tt = tts.into_iter().map_into::<TokenStream>().fold(
                    TokenStream::new(),
                    |mut stream, other| {
                        stream.extend(other);
                        stream
                    },
                );
                quote! { #tt, }
            })
            .fold(TokenStream::new(), |mut stream, other| {
                stream.extend(other);
                stream
            });
        s[7] = quote! {
            { #fields }
        };
    }
}

fn generate_crates(
    abi_file: &str,
    version: &str,
    paths: &Paths,
    structs: &mut BTreeMap<String, Vec<TokenStream>>,
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
        let mut tts = vec![tt.into()];
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
        structs.insert(key, tts);
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
                },
            ),
            (
                "ethers".into(),
                CargoTomlDepMeta {
                    version: ETHERS_VERSION.into(),
                    optional: false,
                },
            ),
            (
                "ethers-contract".into(),
                CargoTomlDepMeta {
                    version: ETHERS_VERSION.into(),
                    optional: false,
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
                },
            ),
            (
                "ethers".into(),
                CargoTomlDepMeta {
                    version: ETHERS_VERSION.into(),
                    optional: false,
                },
            ),
            (
                "ethers-contract".into(),
                CargoTomlDepMeta {
                    version: ETHERS_VERSION.into(),
                    optional: false,
                },
            ),
        ],
        [],
        paths,
    )?;
    generate_crate_source(
        get_subcrate(abi_file, "events"),
        paths,
        std::iter::once(quote! {
            #![allow(dead_code)]
            #![allow(unused_imports)]
            use ::ethbridge_structs::*;
        })
        .chain(abi.events.into_iter().map(|tt| tt.into())),
    )?;
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
                        },
                    )
                },
                || {
                    (
                        "ethbridge-governance-events".into(),
                        CargoTomlDepMeta {
                            version: String::new(),
                            optional: false,
                        },
                    )
                },
            ),
            (
                "ethbridge-structs".into(),
                CargoTomlDepMeta {
                    version: String::new(),
                    optional: false,
                },
            ),
            (
                "ethers".into(),
                CargoTomlDepMeta {
                    version: ETHERS_VERSION.into(),
                    optional: false,
                },
            ),
            (
                "ethers-contract".into(),
                CargoTomlDepMeta {
                    version: ETHERS_VERSION.into(),
                    optional: false,
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
    let err = std::fs::write(
        &cargo_toml_path,
        cargo_template
            .render("Cargo.toml", &tera_context)
            .context("failed to render Cargo.toml")?,
    );
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
