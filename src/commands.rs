//! `cargo repro` subcommands

pub mod build;
pub mod verify;

use self::{build::BuildCommand, verify::VerifyCommand};

/// `cargo repro` subcommands
pub enum Command {
    /// `cargo repro build` subcommand
    Build(BuildCommand),

    /// `cargo repro verify` subcommand
    Verify(VerifyCommand),
}

impl Command {
    /// Parse command to execute from CLI args
    pub fn from_args(mut args: impl Iterator<Item = String>) -> Option<Self> {
        // ARGV[0] is always the name of the executed binary
        args.next().unwrap();

        // Cargo passes `repro` as the first argument when invoking `cargo repro`
        if args.next().as_ref().map(String::as_str) != Some("repro") {
            return None;
        }

        let command = match args.next().as_ref().map(String::as_str) {
            Some("build") => Command::Build(BuildCommand::from_args(args)),
            Some("verify") => Command::Verify(VerifyCommand::from_args(args)),
            _ => return None,
        };

        Some(command)
    }

    /// Run the parsed command
    pub fn run(&self) {
        match self {
            Command::Build(build) => build.run(),
            Command::Verify(verify) => verify.run(),
        }
    }
}
