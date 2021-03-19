//! Rust project builder - wrapper for invoking Cargo

use std::{
    ffi::OsString,
    process::{Child, Command, ExitStatus},
};

/// Name of the `cargo` executable
const CARGO_EXE: &str = "cargo";

/// Rust project builder
#[derive(Clone, Debug)]
pub struct Builder {
    program: OsString,
    args: Vec<OsString>,
}

impl Default for Builder {
    fn default() -> Self {
        Self::new(CARGO_EXE)
    }
}

impl Builder {
    /// Create `Builder` that invokes the given command with the given arguments
    pub fn new<S>(program: S) -> Self
    where
        S: Into<OsString>,
    {
        Self {
            program: program.into(),
            args: vec![],
        }
    }

    /// Append an argument to the set of arguments to run
    pub fn arg<S>(&mut self, arg: S) -> &mut Self
    where
        S: Into<OsString>,
    {
        self.args.push(arg.into());
        self
    }

    /// Append multiple arguments to the set of arguments to run
    pub fn args<I, S>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: Into<OsString>,
    {
        self.args.extend(args.into_iter().map(|a| a.into()));
        self
    }

    /// Run the given subcommand
    pub fn run(&self) -> Process {
        let child = Command::new(&self.program)
            .args(&self.args)
            .spawn()
            .unwrap_or_else(|e| {
                panic!("error running command: {}", e);
            });

        Process(child)
    }
}

/// Wrapper for the builder subprocess
pub struct Process(Child);

impl Process {
    /// Wait for the child to finish
    pub fn wait(mut self) -> ExitStatus {
        self.0
            .wait()
            .unwrap_or_else(|e| panic!("couldn't get child's exit status: {}", e))
    }
}
