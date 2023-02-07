use std::path::PathBuf;

use clap::Parser;
//use ethers_contract::Abigen;

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

    /// The root directory where we output the generated crates into
    #[arg(short = 'o', long, default_value_t = String::from("."))]
    root: String,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
    }
}

fn run() -> eyre::Result<()> {
    let Args {
        root,
        abi_files_dir,
        ethereum_bridge_tag,
    } = Args::parse();

    let root_dir: PathBuf = root.into();
    let abi_files_dir: PathBuf = abi_files_dir.into();
    if let Some(_tag) = ethereum_bridge_tag {
        // TODO: download ABI files
        eyre::bail!("downloading of ABI artifacts is not implemented yet");
    }

    println!("{abi_files_dir:#?}");
    println!("{root_dir:#?}");
    Ok(())
}

//fn generate_crate(krate: &str, abi_files_dir: &PathBuf) ->
