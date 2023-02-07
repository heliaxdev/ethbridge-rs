use std::collections::BTreeMap;
use std::path::PathBuf;

use clap::Parser;
use ethers_contract::Abigen;
use eyre::{eyre as err, WrapErr};
use proc_macro2::TokenTree;

// TODO: dedup types strat:
// - state machine look for: `pub`, `struct`, `$IDENT`
// - if `$IDENT` is in set, then output it;
// - otherwise do not output this type
//
// then:
// - modify imports of other crates to reference the common
// crate
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
    /// If not output directory is specified, the current working
    /// directory is used instead
    #[arg(short = 'o', long)]
    output_dir: Option<String>,

    /// The version of the generated crates. If not specified, the
    /// default value of `0.1.0` is used
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
        ..
    } = Args::parse();

    let paths = Paths {
        output_dir: output_dir.map(|s| s.into()).unwrap_or_else(PathBuf::new),
        abi_files_dir: abi_files_dir.into(),
    };
    if let Some(tag) = ethereum_bridge_tag {
        download_abi_files(tag, &paths)?;
    }

    let mut structs = BTreeMap::new();
    generate_crates("Bridge", &paths, &mut structs)?;
    generate_crates("Governance", &paths, &mut structs)?;

    println!("ABI structs: {structs:#?}");
    Ok(())
}

// TODO: generate one crate per contract
// TODO: common crate with all shared types (e.g. Signature)
fn generate_crates(
    abi_file: &str,
    paths: &Paths,
    structs: &mut BTreeMap<String, Vec<TokenTree>>,
) -> eyre::Result<()> {
    let abi_file_path = {
        let mut path = paths.abi_files_dir.clone();
        path.push(format!("{abi_file}.abi"));
        path
    };
    let output_dir = {
        let mut path = paths.output_dir.clone();
        path.push(abi_file.to_lowercase());
        path
    };
    let abi_gen = Abigen::from_file(&abi_file_path)
        .with_context(|| format!("file not found: {}", abi_file_path.display()))?;
    let (abi, _) = abi_gen.expand()?;
    let mut structs_iter = abi.abi_structs.into_iter();
    loop {
        let Some(tt) = structs_iter.next() else {
            break;
        };
        let mut tts = vec![tt];
        for _ in 0..5 {
            tts.push(
                structs_iter
                    .next()
                    .ok_or_else(|| err!("insufficient token trees in generated rust code"))?,
            );
        }
        let Some(TokenTree::Ident(ident)) = structs_iter.next() else {
            eyre::bail!("expected identifier in generated rust code, but got something else");
        };
        let key = ident.to_string();
        tts.push(TokenTree::Ident(ident));
        tts.push(
            structs_iter
                .next()
                .ok_or_else(|| err!("struct definition not found in generated rust code"))?,
        );
        structs.insert(key, tts);
    }
    Ok(())
}

fn download_abi_files(_tag: String, _paths: &Paths) -> eyre::Result<()> {
    eyre::bail!("downloading of ABI artifacts is not implemented yet")
}
