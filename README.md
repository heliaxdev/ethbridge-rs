# ethbridge-rs

This repository contains the source code of `generate-ethbridge-crates`, whose
purpose, as the name implies, is to automatically generate code to interop with
the Ethereum bridge smart contracts written for Namada.

```
Generate Ethereum bridge Rust types compatible with Namada's Rust code

Usage: generate-ethbridge-crates [OPTIONS]

Options:
  -p, --abi-files-dir <ABI_FILES_DIR>
          The path to the directory to parse ABI files from [default: target]
  -t, --ethereum-bridge-tag <ETHEREUM_BRIDGE_TAG>
          The git tag of `ethereum-bridge` whose artifacts we are to download.
          If no tag is provided, we try to use files present in the specified
          ABI files directory
  -o, --output-dir <OUTPUT_DIR>
          Path to the output directory of the generated crates. If no output
          directory is specified, the current working directory is used instead
  -v, --crate-version <CRATE_VERSION>
          The version of the generated crates. If not specified, the version of
          this CLI command is used instead
  -h, --help
          Print help
  -V, --version
          Print version
```

## Usage

One must first acquire Ethereum ABI JSON files to interop with the smart contracts,
before invoking `generate-ethbridge-crates`, either offline, or via a git tag in
the [`ethereum-bridge`](https://github.com/anoma/ethereum-bridge) repository. If
the git tag way is preferred, the files will be downloaded to and stored in the
`target/` directory of this repository by default. Otherwise, the ABI files must
already be present in the aforementioned directory.

Then, it is a matter of invoking `cargo run --bin generate-ethbridge-crates` in
the command line. The version of the generated crates will be the same of the
CLI util, unless it is overwritten with the `-v <version-string>` flag. Further,
the generated code will not be formatted, so it is advised to run `cargo fmt`
before pushing the generated code to the git remote of this repository.

Additional usage instructions can be consulted with the `-h` command line flag.
