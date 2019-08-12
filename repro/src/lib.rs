//! `repro` crate: perform and verify reproducible builds of Rust code

#![forbid(unsafe_code)]
#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![doc(
    html_logo_url = "https://avatars3.githubusercontent.com/u/44121472",
    html_root_url = "https://docs.rs/repro/0.0.0"
)]

pub mod builder;
