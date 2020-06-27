/*! Functions and templates which can be imported by `app.rs` to save effort */
// Copyright 2017-2019, Stephan Sokolow

use structopt::{clap, StructOpt};

/// Modified version of Clap's default template for proper help2man compatibility
///
/// Used as a workaround for:
/// 1. Clap's default template interfering with `help2man`'s proper function
///    ([clap-rs/clap/#1432](https://github.com/clap-rs/clap/issues/1432))
/// 2. Workarounds involving injecting `\n` into the description breaking help output if used
///    on subcommand descriptions.
pub const HELP_TEMPLATE: &str = "{bin} {version}

{about}

USAGE:
    {usage}

{all-args}
";

#[allow(clippy::missing_docs_in_private_items)]
// Can't doc-comment until TeXitoi/structopt#333
// Options used by boilerplate code in `main.rs`
//
// FIXME: Report that StructOpt trips Clippy's `cast_possible_truncation` lint unless I use
//        `u64` for my `from_occurrences` inputs, which is a ridiculous state of things.
#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
pub struct BoilerplateOpts {
    /// Decrease verbosity (-q, -qq, -qqq, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    pub quiet: u64,

    /// Increase verbosity (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u64,

    /// Display timestamps on log messages (sec, ms, ns, none)
    #[structopt(short, long, value_name = "resolution")]
    pub timestamp: Option<stderrlog::Timestamp>,

    /// Write a completion definition for the specified shell to stdout (bash, zsh, etc.)
    #[structopt(long, value_name = "shell")]
    pub dump_completions: Option<clap::Shell>,
}
