//! `cargo repro verify` subcommand

/// `cargo repro verify` subcommand
pub struct VerifyCommand {
    /// Arguments passed to `cargo repro verify` (to be passed to Cargo)
    pub args: Vec<String>,
}

impl VerifyCommand {
    /// Initialize this command from the given arguments, which should *NOT*
    /// include `["cargo", "repro", "verify"]`
    pub fn from_args(args: impl Iterator<Item = String>) -> Self {
        Self {
            args: args.collect(),
        }
    }

    /// Run this subcommand
    pub fn run(&self) {
        println!("cargo repro: build and verify byte-for-byte reproducible Rust packages");
        println!();
        println!("WORK IN PROGRESS: The 'verify' functionality of this tool is unimplemented.");
        println!("If you are interested in contributing, please see the GitHub issues:");
        println!();
        println!("    https://github.com/rust-secure-code/cargo-repro/issues");
        println!();
    }
}
