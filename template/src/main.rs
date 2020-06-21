/*! TODO: Application description here

This file provided by [rust-cli-boilerplate](https://github.com/ssokolow/rust-cli-boilerplate)
*/
// Copyright 2017-2019, Stephan Sokolow

// `error_chain` recursion adjustment
#![recursion_limit = "1024"]

// Make rustc's built-in lints more strict and set clippy into a whitelist-based configuration so
// we see new lints as they get written (We'll opt back out selectively)
#![warn(warnings, rust_2018_idioms, unsafe_code)]
#![warn(clippy::all, clippy::pedantic, clippy::restriction)]

// Opt out of the lints I've seen and don't want
#![allow(clippy::float_arithmetic, clippy::implicit_return, clippy::needless_return)]

// stdlib imports
use std::io;
use std::convert::TryInto;

// 3rd-party imports
mod errors;
use structopt::{clap, StructOpt};
use log::error;

// Local imports
mod app;
mod helpers;
mod validators;

/// Boilerplate to parse command-line arguments, set up logging, and handle bubbled-up `Error`s.
///
/// Based on the `StructOpt` example from stderrlog and the suggested error-chain harness from
/// [quickstart.rs](https://github.com/brson/error-chain/blob/master/examples/quickstart.rs).
///
/// See `app::main` for the application-specific logic.
///
/// **TODO:** Consider switching to Failure and look into `impl Termination` as a way to avoid
///           having to put the error message pretty-printing inside main()
fn main() {
    // Parse command-line arguments (exiting on parse error, --version, or --help)
    let opts = app::CliOpts::from_args();

    // Configure logging output so that -q is "decrease verbosity" rather than instant silence
    let verbosity = opts.boilerplate.verbose
                        .saturating_add(app::DEFAULT_VERBOSITY)
                        .saturating_sub(opts.boilerplate.quiet);

    #[allow(clippy::result_expect_used)]
    stderrlog::new()
        .module(module_path!())
        .quiet(verbosity == 0)
        .verbosity(verbosity.saturating_sub(1).try_into().expect("should never even come close"))
        .timestamp(opts.boilerplate.timestamp.unwrap_or(stderrlog::Timestamp::Off))
        .init()
        .expect("initializing logging output");

    // If requested, generate shell completions and then exit with status of "success"
    if let Some(shell) = opts.boilerplate.dump_completions {
        app::CliOpts::clap().gen_completions_to(
            app::CliOpts::clap().get_bin_name().unwrap_or_else(|| clap::crate_name!()),
            shell,
            &mut io::stdout());
        std::process::exit(0);
    };

    if let Err(ref e) = app::main(opts) {
        // Write the top-level error message, then chained errors, then backtrace if available
        error!("error: {}", e);
        for e in e.iter().skip(1) {
            error!("caused by: {}", e);
        }
        if let Some(backtrace) = e.backtrace() {
            error!("backtrace: {:?}", backtrace);
        }

        // Exit with a nonzero exit code
        // TODO: Decide how to allow code to set this to something other than 1
        std::process::exit(1);
    }
}

// vim: set sw=4 sts=4 expandtab :
