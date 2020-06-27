/*! TODO: Application description here

This file provided by [rust-cli-boilerplate](https://github.com/ssokolow/rust-cli-boilerplate)
*/
// Copyright 2017-2020, Stephan Sokolow

// Make rustc's built-in lints more strict and set clippy into a whitelist-based configuration so
// we see new lints as they get written, then opt out of ones we have seen and don't want
#![warn(warnings, rust_2018_idioms)]
#![warn(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(clippy::float_arithmetic, clippy::implicit_return, clippy::needless_return)]
#![forbid(unsafe_code)] // Enforce my policy of only allowing it in my own code as a last resort

// stdlib imports
use std::convert::TryInto;
use std::io;

// 3rd-party imports
use anyhow::{Context, Result};
use structopt::{clap, StructOpt};

// Local imports
mod app;
mod helpers;
mod validators;

/// Boilerplate to parse command-line arguments, set up logging, and handle bubbled-up `Error`s.
///
/// See `app::main` for the application-specific logic.
fn main() -> Result<()> {
    // Parse command-line arguments (exiting on parse error, --version, or --help)
    let opts = app::CliOpts::from_args();

    // Configure logging output so that -q is "decrease verbosity" rather than instant silence
    let verbosity = opts
        .boilerplate
        .verbose
        .saturating_add(app::DEFAULT_VERBOSITY)
        .saturating_sub(opts.boilerplate.quiet);

    stderrlog::new()
        .module(module_path!())
        .quiet(verbosity == 0)
        .verbosity(verbosity.saturating_sub(1).try_into().context("Verbosity too high")?)
        .timestamp(opts.boilerplate.timestamp.unwrap_or(stderrlog::Timestamp::Off))
        .init()
        .context("Failed to initialize logging output")?;

    // If requested, generate shell completions and then exit with status of "success"
    if let Some(shell) = opts.boilerplate.dump_completions {
        app::CliOpts::clap().gen_completions_to(
            app::CliOpts::clap().get_bin_name().unwrap_or_else(|| clap::crate_name!()),
            shell,
            &mut io::stdout(),
        );
        Ok(())
    } else {
        // Run the actual `main` and rely on `impl Termination` to provide a simple, concise way to
        // allow terminal errors that can be changed later as needed but starts out analogous to
        // letting an unhandled exception bubble up in something like Python.
        // TODO: Experiment with this and look for ways to polish it up further
        app::main(opts)
    }
}

// vim: set sw=4 sts=4 expandtab :
