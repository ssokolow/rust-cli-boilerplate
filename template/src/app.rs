/*! Application-specific logic lives here

    **TODO:** Look into moving the argument definition into a
    [build.rs](https://doc.rust-lang.org/cargo/reference/build-scripts.html) like in the
    [clap_generate](https://docs.rs/clap_generate/3.0.0-beta.1/clap_generate/fn.generate_to.html)
    examples so I don't have build the completion generation code into the output binary.
*/

// Parts Copyright 2017-2020, Stephan Sokolow

// Standard library imports
use std::path::PathBuf;

// 3rd-party crate imports
use anyhow::Result;
use structopt::StructOpt;

#[allow(unused_imports)] // TEMPLATE:REMOVE
use log::{debug, error, info, trace, warn};

// Local Imports
use crate::helpers::{BoilerplateOpts, HELP_TEMPLATE};
use crate::validators::path_readable_file;

/// The verbosity level when no `-q` or `-v` arguments are given, with `0` being `-q`
pub const DEFAULT_VERBOSITY: u64 = 1;

/// Command-line argument schema
///
/// ## Relevant Conventions:
///
///  * Make sure that there is a blank space between the `<name>` `<version>` line and the
///    description text or the `--help` output won't comply with the platform conventions that
///    `help2man` depends on to generate your manpage.
///    (Specifically, it will mistake the `<name> <version>` line for part of the description.)
///  * `StructOpt`'s default behaviour of including the author name in the `--help` output is an
///    oddity among Linux commands and, if you don't disable it, you run the risk of people
///    unfamiliar with `StructOpt` assuming that you are an egotistical person who made a conscious
///    choice to add it.
///
///    The proper standardized location for author information is the `AUTHOR` section which you
///    can read about by typing `man help2man`.
///
/// ## Cautions:
///  * Subcommands do not inherit `template` and it must be re-specified for each one.
///    ([clap-rs/clap#1184](https://github.com/clap-rs/clap/issues/1184))
///  * Double-check that your choice of `about` or `long_about` is actually overriding this
///    doc comment. The precedence is affected by things you wouldn't expect, such as the presence
///    or absence of `template` and it's easy to wind up with this doc-comment as your `--help`
///    ([TeXitoi/structopt#173](https://github.com/TeXitoi/structopt/issues/173))
///  * Do not begin the description text for subcommands with `\n`. It will break the formatting in
///    the top-level help output's list of subcommands.
#[derive(StructOpt, Debug)]
#[structopt(template = HELP_TEMPLATE,
            about = "TODO: Replace me with the description text for the command",
            global_setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct CliOpts {
    #[allow(clippy::missing_docs_in_private_items)] // StructOpt compile-time errors if we doc this
    #[structopt(flatten)]
    pub boilerplate: BoilerplateOpts,

    // -- Arguments used by application-specific logic --

    /// File(s) to use as input
    ///
    /// **TODO:** Figure out if there's a way to only enforce constraints on this when not asking
    ///           to dump completions.
    #[structopt(parse(from_os_str),
                validator_os = path_readable_file)]
    inpath: Vec<PathBuf>,
}

/// The actual `main()`
pub fn main(opts: CliOpts) -> Result<()> {
    #[allow(unused_variables, clippy::unimplemented)] // TEMPLATE:REMOVE
    for inpath in opts.inpath {
        todo!("Implement application logic")
    }

    Ok(())
}

// Tests go below the code where they'll be out of the way when not the target of attention
#[cfg(test)]
mod tests {
    #[allow(unused_imports)] // TEMPLATE:REMOVE
    use super::CliOpts;

    // TODO: Unit test to verify that the doc comments on `CliOpts` or `BoilerplateOpts` aren't
    // overriding the intended about string.

    #[test]
    /// Test something
    fn test_something() {
        // TODO: Test something
    }
}
