//! cargo-repro: perform and verify reproducible builds of Rust code with Cargo

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

pub mod commands;

use self::commands::Command;
use std::{env, process};

fn main() {
    let command = Command::from_args(env::args()).unwrap_or_else(|| usage());
    command.run();
}

fn usage() -> ! {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!(
        "{}\n",
        env!("CARGO_PKG_DESCRIPTION")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    );

    println!("SUBCOMMANDS:");
    println!("    build\tPerform a reproducible build of a Cargo project");
    println!("    verify\t(UNIMPLEMENTED) Verify a reproducible build");

    process::exit(1);
}
