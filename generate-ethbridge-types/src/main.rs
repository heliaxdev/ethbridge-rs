use std::path::PathBuf;

use clap::Parser;
use ethers_contract::Abigen;
use eyre::WrapErr;

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
    } = Args::parse();

    let paths = Paths {
        output_dir: output_dir.map(|s| s.into()).unwrap_or_else(PathBuf::new),
        abi_files_dir: abi_files_dir.into(),
    };
    if let Some(tag) = ethereum_bridge_tag {
        download_abi_files(tag, &paths)?;
    }

    generate_crate("Bridge", &paths)
}

// TODO: generate one crate per contract
// TODO: common crate with all shared types (e.g. Signature)
fn generate_crate(abi_file: &str, paths: &Paths) -> eyre::Result<()> {
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
    println!("Output: {output_dir:#?}");
    println!("Path: {abi_file_path:#?}");
    let abi_gen = Abigen::from_file(&abi_file_path)
        .with_context(|| format!("file not found: {}", abi_file_path.display()))?;
    let (abi, _abi_ctx) = abi_gen.expand()?;
    println!("ABI structs: {:#?}", abi.abi_structs);
    Ok(())
}

fn download_abi_files(_tag: String, _paths: &Paths) -> eyre::Result<()> {
    eyre::bail!("downloading of ABI artifacts is not implemented yet")
}
