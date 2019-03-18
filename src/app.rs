/*! Application-specific logic lives here */
// Parts Copyright 2017-2019, Stephan Sokolow

// Standard library imports
use std::path::{Component::CurDir, PathBuf};

// 3rd-party crate imports
use structopt::{clap, StructOpt};

#[allow(unused_imports)] // TEMPLATE:REMOVE
use log::{debug, error, info, trace, warn};

// Local Imports
use crate::errors::*;
use crate::validators::path_readable;

/// The verbosity level when no `-q` or `-v` arguments are given, with `0` being `-q`
pub const DEFAULT_VERBOSITY: usize = 1;

/// Command-line argument schema
///
/// ## Relevant Conventions:
///  * The top-level `long_about` attribute should begin with `\n` or the `--help` output won't
///    comply with the platform conventions that `help2man` depends on to generate your manpage.
///    (Specifically, it will mistake the `<name> <version>` line for part of the description.)
///  * StructOpt's default behaviour of including the author name in the `--help` output is an
///    oddity among Linux commands and, if you don't disable it with `author=""`, you run the risk
///    of people unfamiliar with `StructOpt` assuming that you are an egotistical person who made a
///    conscious choice to add it.
///
/// ## Cautions:
///  * If you use `about` rather than `long_about`, this docstring will be displayed in your
///  `--help` output.
///  * As of this writing, there is a bug which will cause you to either have no leading `\n` or a
///    doubled leading `\n` if you write your `--help` description as a doc comment rather than
///    using `long_about`.
#[derive(StructOpt, Debug)]
#[structopt(author="", rename_all = "kebab-case",
            long_about = "\nTODO: Replace me with the description text for the command",
            raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
pub struct CliOpts {
    /// Decrease verbosity (-q, -qq, -qqq, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    pub quiet: usize,
    /// Increase verbosity (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: usize,
    /// Display timestamps on log messages (sec, ms, ns, none)
    #[structopt(short, long, value_name = "resolution")]
    pub timestamp: Option<stderrlog::Timestamp>,
    /// Write a completion definition for the specified shell to stdout (bash, zsh, etc.)
    #[structopt(long, value_name = "shell")]
    pub dump_completions: Option<clap::Shell>,

    /// File(s) to use as input
    #[structopt(parse(from_os_str),
                raw(validator_os = "path_readable", default_value_os = "CurDir.as_os_str()"))]
    inpath: Vec<PathBuf>,
}

/// The actual `main()`
pub fn main(opts: CliOpts) -> Result<()> {
    #[allow(unused_variables, clippy::unimplemented)] // TEMPLATE:REMOVE
    for inpath in opts.inpath {
        unimplemented!()
    }

    Ok(())
}

// Tests go below the code where they'll be out of the way when not the target of attention
#[cfg(test)]
mod tests {
    #[allow(unused_imports)] // TEMPLATE:REMOVE
    use super::CliOpts;

    #[test]
    /// Test something
    fn test_something() {
        // TODO: Test something
    }
}
