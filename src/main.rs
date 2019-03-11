/*! TODO: Application description here

*/

// `error_chain` recursion adjustment
#![recursion_limit = "1024"]

// Make rustc's built-in lints more strict and set clippy into a whitelist-based configuration so
// we see new lints as they get written (We'll opt back out selectively)
#![warn(warnings, clippy::all, clippy::complexity, clippy::correctness, clippy::pedantic,
        clippy::perf, clippy::style, clippy::restriction)]

// Opt out of the lints I've seen and don't want
#![allow(clippy::float_arithmetic)]

/// The verbosity level when no `-q` or `-v` arguments are given, with `0` being `-q`
const DEFAULT_VERBOSITY: usize = 1;

// stdlib imports
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::path::{Component::CurDir, Path, PathBuf};

// `error_chain`, `structopt`, and logging imports
mod errors;
use crate::errors::*;
use log::{debug, error, info, trace, warn};
use structopt::StructOpt;

/// Command-line argument schema
///
/// **NOTE:** The top-level `about` should begin with a newline (`\n`) or the resulting `--help`
///           won't comply with platform conventions and tools like help2man will treat the
///           "<name> <version>" line as part of `about`.
#[derive(StructOpt, Debug)]
#[structopt(author="", long_about = "\nTODO: Replace me with the description text for the command",
            raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
struct Opt {
    /// Decrease verbosity (-q, -qq, -qqq, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    quiet: usize,
    /// Increase verbosity (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: usize,
    /// Display timestamps on log messages (sec, ms, ns, none)
    #[structopt(short, long)]
    timestamp: Option<stderrlog::Timestamp>,

    /// File(s) to use as input
    #[structopt(parse(from_os_str),
                raw(validator_os = "path_readable", default_value_os = "CurDir.as_os_str()"))]
    inpath: Vec<PathBuf>,
}

/// Clap/StructOpt validator for testing that the given path can be opened for reading
fn path_readable(value: &OsStr) -> std::result::Result<(), OsString> {
    File::open(&value)
        .map(|_| ())
        .map_err(|e| format!("{}: {}", Path::new(value).display(), e).into())
}

/// Boilerplate to parse command-line arguments, set up logging, and handle bubbled-up `Error`s.
///
/// Based on the `StructOpt` example from stderrlog and the suggested error-chain harness from
/// [quickstart.rs](https://github.com/brson/error-chain/blob/master/examples/quickstart.rs).
///
/// **TODO:** Consider switching to Failure and look into `impl Termination` as a way to avoid
///           having to put the error message pretty-printing inside main()
fn main() {
    // Parse command-line arguments (exiting on parse error, --version, or --help)
    let opts = Opt::from_args();

    // Configure logging output so that -q is "decrease verbosity" rather than instant silence
    let verbosity = (opts.verbose.saturating_add(DEFAULT_VERBOSITY)).saturating_sub(opts.quiet);
    stderrlog::new()
        .module(module_path!())
        .quiet(verbosity == 0)
        .verbosity(verbosity.saturating_sub(1))
        .timestamp(opts.timestamp.unwrap_or(stderrlog::Timestamp::Off))
        .init()
        .expect("initializing logging output");

    if let Err(ref e) = run(opts) {
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

/// The actual `main()`
fn run(opts: Opt) -> Result<()> {
    for inpath in opts.inpath {
        unimplemented!()
    }

    Ok(())
}

// Tests go below the code where they'll be out of the way when not the target of attention
#[cfg(test)]
mod tests {
    // use super::Opt;

    #[test]
    /// Test something
    fn test_something() {
        unimplemented!();
    }
    // TODO: Unit tests
}

// vim: set sw=4 sts=4 expandtab :
