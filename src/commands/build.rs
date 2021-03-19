//! `cargo repro build` subcommand

use repro::builder::Builder;

/// Cargo argument for a locked build. This is needed to ensure the build
/// is reproducible.
pub const LOCKED_ARG: &str = "--locked";

/// `cargo repro build` subcommand
pub struct BuildCommand {
    /// Arguments passed to `cargo repro build` (to be passed to Cargo)
    pub args: Vec<String>,
}

impl BuildCommand {
    /// Initialize this command from the given arguments, which should *NOT*
    /// include `["cargo", "repro", "build"]`
    pub fn from_args(args: impl Iterator<Item = String>) -> Self {
        Self {
            args: args.collect(),
        }
    }

    /// Run this subcommand
    // TODO(tarcieri): factor more of this logic into the `repro` crate?
    pub fn run(&self) {
        let mut builder = Builder::default();
        builder.arg("build");

        // Add the `--locked` argument unless it's been specified explicitly
        if !self.args.iter().any(|arg| arg.as_str() == LOCKED_ARG) {
            builder.arg(LOCKED_ARG);
        }

        builder.args(&self.args);
        let exit_status = builder.run().wait();

        if !exit_status.success() {
            panic!(
                "cargo exited with non-zero status: {}",
                exit_status
                    .code()
                    .map(|code| code.to_string())
                    .unwrap_or_else(|| "unknown".to_owned())
            );
        }
    }
}
