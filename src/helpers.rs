/*! Functions and templates which can be imported by app.rs to save effort */
// Copyright 2017-2019, Stephan Sokolow

use structopt::{clap, StructOpt};

/// Modified version of Clap's default template for proper help2man compatibility
///
/// Used as a workaround for:
/// 1. StructOpt making display of `author` opt-out and not inheriting your preference in
///    subcommands. ([TeXitoi/structopt/#172](https://github.com/TeXitoi/structopt/issues/172))
/// 2. Clap's default template interfering with `help2man`'s proper function
///    ([clap-rs/clap/#1432](https://github.com/clap-rs/clap/issues/1432))
/// 3. Workarounds involving injecting `\n` into the description breaking help output if used
///    on subcommand descriptions.
pub const HELP_TEMPLATE: &str = "{bin} {version}

{about}

USAGE:
    {usage}

{all-args}
";

/// Options used by boilerplate code
// TODO: Move these into a struct of their own in something like helpers.rs
#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
pub struct BoilerplateOpts {
    // -- Arguments used by main.rs --
    // TODO: Move these into a struct of their own in something like helpers.rs

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
}

