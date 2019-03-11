//! TODO: Application description here
//!
//! # Development Policy
//! Clap validators for references like filesystem paths (as opposed to self-contained
//! data like set sizes) are to be used only to improving the user experience by
//! maximizing the chance that bad data will be caught early.
//!
//! To avoid vulnerabilities based on race conditions or shortcomings in functions like
//! access() (which may falsely claim "/" is writable), all "reference data" must be
//! validated (and failures handled) on **every** use.
//!
//! See Also:
//!  http://blog.ssokolow.com/archives/2016/10/17/a-more-formal-way-to-think-about-validity-of-input-data/

// `error_chain` recursion adjustment
#![recursion_limit = "1024"]

// Make rustc's built-in lints more strict (I'll opt back out selectively)
#![warn(warnings)]

// Set clippy into a whitelist-based configuration so I'll see new lints as they come in
#![warn(clippy::all, clippy::complexity, clippy::correctness, clippy::pedantic,
        clippy::perf, clippy::style, clippy::restriction)]

// Opt out of the lints I've seen and don't want
#![allow(clippy::assign_ops, clippy::float_arithmetic)]

/// `error_chain` imports
mod errors;
use crate::errors::*;

/// clap-rs imports
use clap::{App, Arg, crate_version};

// TODO: Logging

/// stdlib imports
use std::path::Component;

/// Slightly adjusted version of the suggested error-chain harness from
/// https://github.com/brson/error-chain/blob/master/examples/quickstart.rs
fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let stderr_fail_msg = "Error writing to stderr";

        // Write the top-level error message
        writeln!(stderr, "error: {}", e).expect(stderr_fail_msg);

        // Trace back through the chained errors
        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(stderr_fail_msg);
        }

        // Print the backtrace if available
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(stderr_fail_msg);
        }

        // Exit with a nonzero exit code
        // TODO: Decide how to allow code to set this to something other than 1
        ::std::process::exit(1);
    }
}

/// The actual main(), but with the ability to use ? for easy early return
fn run() -> Result<()> {
    // env::current_dir is fallible and default_value can't take a callback for lazy eval, so
    // resort to "." but future-proof it in case of esoteric platforms.
    // (Not perfect, but to_string_lossy() is necessary without a `default_value_os`)
    let lazy_currdir = &Component::CurDir.as_os_str().to_string_lossy();

    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(crate_version!())
        // .about("Description text here")
        // TODO: Add args to control logging level
        .arg(Arg::with_name("inpath")
            .help("File(s) to use as input")
            .multiple(true)
            .empty_values(false)
            .default_value(lazy_currdir)
            // .validator_os(validators::path_readable)
            .required(true))
        .get_matches();

    for inpath in matches.values_of_os("inpath").expect("unreachable: Arg.required(true)") {
        unimplemented!()
    }

    Ok(())
}

// Tests go below the code where they'll be out of the way when not the target of attention
#[cfg(test)]
mod tests {
    // use super::make_clap_parser;

    #[test]
    /// Test something
    fn test_something() {
        unimplemented!();
    }
    // TODO: Unit tests
}

// vim: set sw=4 sts=4 expandtab :
