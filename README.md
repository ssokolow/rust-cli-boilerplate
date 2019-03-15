# Rust CLI Project Template
![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache_2.0-blue.svg)
![POSIX-only build tooling](https://img.shields.io/badge/dev_platform-POSIX-lightgrey.svg)

A base project template for building small but reliable utilities in the
[Rust](https://rust-lang.org/) programming language.

**NOTE:** While the `LICENSE` file must contain my preferred choice for
starting new projects (the GNU GPLv3), **you may use the contents of this
repository under your choice of the [MIT](http://opensource.org/licenses/MIT)
and/or [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0) licenses**.

## Features

* Uses [StructOpt](https://github.com/TeXitoi/structopt) (with colorized
  `--help` output and "Did you mean...?" suggestions enabled) for argument
  parsing.
* Uses [error-chain](https://github.com/rust-lang-nursery/error-chain) for
  unified error handling.
* Presents a `run(opts: Opt)` function for you to use as your `main()` to
  keep the boilerplate cleanly separated from the specific logic.
* Exposes Clap's support for generating shell completions by providing a
  `--dump-completions <shell>` option.
* Enables almost all rustc and
  [clippy](https://github.com/rust-lang/rust-clippy) lints without making
  clippy mandatory.
* A comprehensive set of [just](https://github.com/casey/just) commands, easily
  customized via variables (eg. for cross-compilation), including `install` and
  `uninstall`, which also take care of shell completions and a manpage.
* `just build-release` for a 100% static i686 binary totalling roughly `252KiB`
  (`228KiB` with `panic="abort"`) in new projects
* `just install-deps` to install all but two optional dependencies on
  Debian-family distros.
* `just install-cargo-deps` to install all distro-agnostic dependencies.
* A basic `.travis.yml` for use with [Travis-CI](https://travis-ci.org/) and
  [Nightli.es](https://nightli.es/).
* The `fmt` just command always calls the nightly version of rustfmt to ensure
  access to the excessive number of customization options which are gated away
  as unstable.

## Supplementary Files

<html>
<table>
<tr><th colspan="2">Metadata</th></tr>
<tr>
  <td><code>LICENSE</code></td>
  <td>A copy of the <a href="https://www.gnu.org/licenses/gpl-3.0.html">GNU GPLv3</a> as my "until I've had time to think about it"
license of choice. You can replace this</td>
</tr>
<tr>
  <td><code>CONTRIBUTING.md</code></td>
  <td>A copy of the
  <a href="https://developercertificate.org/">Developer Certificate of Origin</a>,
  suitable for both this template and projects generated from it, which is the
  Linux kernel developers' more ideologically appropriate alternative to CLAs
  as a means of legally armoring themselves against bad-faith
  contributions</td>
</tr>
<tr><th colspan="2">Configuration</th></tr>
<tr>
  <td><code>.gitignore</code></td>
  <td>Just ignore <code>/target</code> since that's where Cargo puts everything</td>
</tr>
<tr>
  <td><code>clippy.toml</code></td>
  <td>Whitelist for CamelCase names which trigger Clippy's "identifier needs
  backticks" lint</td>
</tr>
<tr>
  <td><code>rustfmt.toml</code></td>
  <td>A custom rustfmt configuration which shows TODO/FIXME comments and
  attempts to make it conform to the style I'm willing to enforce at the
  expense of not using rustfmt if necessary.</td>
</tr>
<tr><th colspan="2">Development Automation</th></tr>
<tr>
  <td><code>apply.py</code></td>
  <td>Run this to generate new projects as a workaround for cargo-generate's
  incompatibility with justfile syntax</td>
</tr>
<tr>
  <td><code>justfile</code></td>
  <td>Build/development-automation commands via <a href="https://github.com/casey/just">just</a> (a pure-Rust make-alike)</td>
</tr>
</table>
</html>

## Justfile Reference

### Variables (`just --evaluate`)
<html>
<!-- BEGIN JUSTFILE TABLE: variables -->
<table>
<tr><th>Variable</th><th>Default Value</th><th>Description</th></tr>
<tr>
  <td><code>CARGO_BUILD_TARGET</code></td>
  <td><code>i686-unknown-linux-musl</code></td>
  <td>Used for <code>cargo</code> commands and installed by
  <code>install-rustup-deps</code></td>
</tr>
<tr>
  <td><code>build_flags</code></td>
  <td><code>--release</code></td>
  <td>An easy place to modify the build flags used</td>
</tr>
<tr>
  <td><code>channel</code></td>
  <td><code>stable</code></td>
  <td>An easy way to override the <code>cargo</code> channel for just this project</td>
</tr>
<tr>
  <td><code>features</code></td>
  <td></td>
  <td>Extra cargo features to enable</td>
</tr>
<tr><th colspan="3"><code>build-release</code></th></tr>
<tr>
  <td><code>sstrip_bin</code></td>
  <td><code>sstrip</code></td>
  <td>Set this if you need to override for a cross-compiling <code>sstrip</code></td>
</tr>
<tr>
  <td><code>strip_bin</code></td>
  <td><code>strip</code></td>
  <td>Set this to the cross-compiler's <code>strip</code> when cross-compiling</td>
</tr>
<tr>
  <td><code>strip_flags</code></td>
  <td><code>--strip-unneeded</code></td>
  <td>Flags passed to <code>strip_bin</code></td>
</tr>
<tr>
  <td><code>upx_flags</code></td>
  <td><code>--ultra-brute</code></td>
  <td>Flags passed to <a href="https://upx.github.io/">UPX</a></td>
</tr>
<tr><th colspan="3"><code>kcachegrind</code></th></tr>
<tr>
  <td><code>callgrind_args</code></td>
  <td></td>
  <td>Extra arguments to pass to <a
  href="http://valgrind.org/docs/manual/cl-manual.html">callgrind</a>.</td>
</tr>
<tr>
  <td><code>callgrind_out_file</code></td>
  <td><code>callgrind.out.justfile</code></td>
  <td>Temporary file used by <code>just kcachegrind</code></td>
</tr>
<tr><th colspan="3"><code>install</code> and <code>uninstall</code></th></tr>
<tr>
  <td><code>bash_completion_dir</code></td>
  <td><code>~/.bash_completion.d</code></td>
  <td>Where to <code>install</code> bash completions. <strong>You'll need to manually
  add some lines to source these files in <code>.bashrc.</code></strong></td>
</tr>
<tr>
  <td><code>fish_completion_dir</code></td>
  <td><code>~/.config/fish/completions</code></td>
  <td>Where to <code>install</code> fish completions. You'll probably never need to
  change this.</td>
</tr>
<tr>
  <td><code>manpage_dir</code></td>
  <td><code>~/.cargo/share/man/man1</code></td>
  <td>Where to <code>install</code> manpages. As long as <code>~/.cargo/bin</code> is
  in your <code>PATH</code>, <code>man</code> should automatically pick up this
  location.</td>
</tr>
<tr>
  <td><code>zsh_completion_dir</code></td>
  <td><code>~/.zsh/functions</code></td>
  <td>Where to <code>install</code> zsh completions. <strong>You'll need to add this
  to your <code>fpath</code> manually</strong></td>
</tr>
</table>
<!-- END JUSTFILE TABLE: variables -->
</html>

### Commands (`just --list`)

**NOTE:** Commands marked with &dagger; will have their behaviour affected by
one or more of the variables listed above.

<html>
<!-- BEGIN JUSTFILE TABLE: commands -->
<table>
<tr><th>Command</th><th>Arguments</th><th>Description</th></tr>
<tr>
  <td><code>DEFAULT</code></td>
  <td></td>
  <td>Shorthand for <code>just test</code></td>
</tr>
<tr><th colspan="3">Development</th></tr>
<tr>
  <td><code>bloat</code></td>
  <td>args&nbsp;(optional)<sub>&dagger;</sub></td>
  <td>Alias for <code>cargo bloat</code></td>
</tr>
<tr>
  <td><code>check</code></td>
  <td>args&nbsp;(optional)<sub>&dagger;</sub></td>
  <td>Alias for <code>cargo check</code></td>
</tr>
<tr>
  <td><code>clean</code></td>
  <td>args&nbsp;(optional)<sub>&dagger;</sub></td>
  <td>Superset of <code>cargo clean -v</code> which deletes other stuff this justfile
  builds</td>
</tr>
<tr>
  <td><code>doc</code></td>
  <td>args&nbsp;(optional)<sub>&dagger;</sub></td>
  <td>Run rustdoc with <code>--document-private-items</code> and then run
  cargo-deadlinks</td>
</tr>
<tr>
  <td><code>fmt</code></td>
  <td>args&nbsp;(optional)<sub>&dagger;</sub></td>
  <td>Alias for <code>cargo +nightly fmt -- {{args}}</code></td>
</tr>
<tr>
  <td><code>fmt-check</code></td>
  <td>args&nbsp;(optional)<sub>&dagger;</sub></td>
  <td>Alias for <code>cargo +nightly fmt -- --check {{args}}</code> which un-bloats
  TODO/FIXME warnings</td>
</tr>
<tr>
  <td><code>kcachegrind</code></td>
  <td>args&nbsp;(optional)<sub>&dagger;</sub></td>
  <td>Run a debug build under <a
  href="http://valgrind.org/docs/manual/cl-manual.html">callgrind</a>, then open
  the profile in <a href="https://kcachegrind.github.io/">KCachegrind</a></td>
</tr>
<tr>
  <td><code>kcov</code></td>
  <td><sub>&dagger;</sub></td>
  <td>Generate a statement coverage report in <code>target/cov/</code></td>
</tr>
<tr>
  <td><code>test</code></td>
  <td><sub>&dagger;</sub></td>
  <td>Run all installed static analysis, plus <code>cargo test</code></td>
</tr>
<tr><th colspan="3">Local Builds</th></tr>
<tr>
  <td><code>build</code></td>
  <td><sub>&dagger;</sub></td>
  <td>Alias for <code>cargo build</code></td>
</tr>
<tr>
  <td><code>install</code></td>
  <td><sub>&dagger;</sub></td>
  <td>Install the un-packed binary, shell completions, and a manpage</td>
</tr>
<tr>
  <td><code>run</code></td>
  <td>args&nbsp;(optional)<sub>&dagger;</sub></td>
  <td>Alias for <code>cargo run -- {{args}}</code></td>
</tr>
<tr>
  <td><code>uninstall</code></td>
  <td><sub>&dagger;</sub></td>
  <td>Remove any files installed by the <code>install</code> task (but leave any
  parent directories created)</td>
</tr>
<tr><th colspan="3">Release Builds</th></tr>
<tr>
  <td><code>build-release</code></td>
  <td><sub>&dagger;</sub></td>
  <td>Call <code>build</code> and then strip and compress the resulting binary</td>
</tr>
<tr>
  <td><code>dist</code></td>
  <td><sub>&dagger;</sub></td>
  <td>Call <code>dist-supplemental</code> and <code>build-release</code> and copy the
  packed binary to <code>dist/</code></td>
</tr>
<tr>
  <td><code>dist-supplemental</code></td>
  <td><sub>&dagger;</sub></td>
  <td>Build the shell completions and a manpage, and put them in <code>dist/</code></td>
</tr>
<tr><th colspan="3">Dependencies</th></tr>
<tr>
  <td><code>install-apt-deps</code></td>
  <td></td>
  <td>Use <code>apt-get</code> to install dependencies <code>cargo</code> can't
  (except <code>kcov</code> and <code>sstrip</code>)</td>
</tr>
<tr>
  <td><code>install-cargo-deps</code></td>
  <td><sub>&dagger;</sub></td>
  <td><code>install-rustup-deps</code> and then <code>cargo install</code> tools</td>
</tr>
<tr>
  <td><code>install-deps</code></td>
  <td></td>
  <td>Run <code>install-apt-deps</code> and <code>install-cargo-deps</code>. List what
  remains.</td>
</tr>
<tr>
  <td><code>install-rustup-deps</code></td>
  <td><sub>&dagger;</sub></td>
  <td>Install (don't update) nightly and <code>channel</code> toolchains, plus
  <code>CARGO_BUILD_TARGET</code>, clippy, and rustfmt</td>
</tr>
</table>
<!-- END JUSTFILE TABLE: commands -->
</html>


### Tips

* Edit the `DEFAULT` command. That's what it's there for.
* You can use `just` from any subdirectory in your project. It's like `git` that way.
* `just path/to/project/` (note the trailing slash) is equivalent to `(cd path/to/project; just)`
* `just path/to/project/command` is equivalent to `(cd path/to/project; just command)`

* The simplest way to activate the bash completion installed by `just install`
  is to add this to your `.bashrc`:

  ```sh
  for script in ~/.bash_completion.d/*; do
    . "$script"
  done
  ```

* The simplest way to activate the zsh completion installed by `just install`
  is to add this to your `.zshrc`:

  ```zsh
  fpath=(~/.zsh/functions(:A) $fpath)
  ```

* Only use Clap/StructOpt validators for references like filesystem paths (as opposed to
  self-contained data like set sizes) as a way to bail out early on bad data,
  not as your *only* check of validity. See [this blog post
 ](http://blog.ssokolow.com/archives/2016/10/17/a-more-formal-way-to-think-about-validity-of-input-data/) for more.

## Build Behaviour

In order to be as suitable as possible for building self-contained,
high-reliability replacements for shell scripts, the following build options
are defined:

### If built via `cargo build`:

1. Backtrace support will be disabled in `error-chain` unless explicitly
   built with the `backtrace` feature. (This began as a workaround to unbreak
   cross-compiling to musl-libc and ARM after backtrace-rs 0.1.6 broke it, but
   it also makes sense to opt out of it if you're using `panic="abort"` to save
   space)

### If built via `cargo build --release`:

1. Unless otherwise noted, all optimizations listed above.
2. Link-time optimization will be enabled (`lto = true`)
3. The binary will be built with `opt-level = "z"` to further reduce file size.
4. Optionally (uncomment a line in `Cargo.toml`) panic via `abort` rather than
   unwinding to allow backtrace code to be pruned away by dead code
   optimization.

### If built via `just build-release`:

1. Unless otherwise noted, all [optimizations
   ](https://lifthrasiir.github.io/rustlog/why-is-a-rust-executable-large.html)
   listed above.
2. The binary will be statically linked against
   [musl-libc](http://www.musl-libc.org/) for maximum portability.
3. The binary will be stripped with [`--strip-unneeded`
   ](https://www.technovelty.org/linux/stripping-shared-libraries.html)
   and then with
   [`sstrip`](http://www.muppetlabs.com/~breadbox/software/elfkickers.html)
   (a [more aggressive](https://github.com/BR903/ELFkickers/tree/master/sstrip)
   companion used in embedded development) to produce the smallest possible
   pre-compression size.
4. The binary will be compressed via
   [`upx --ultra-brute`](https://upx.github.io/).
   In my experience, this makes a file about 1/3rd the size of the input.

**NOTE:** `--strip-unneeded` removes all symbols that `readelf --syms` sees
   from the `just build` output, so it's not different from `--strip-all` in
   this case, but it's a good idea to get in the habit of using the safe option
   that's smart enough to just Do What I Mean™.

### If built by `just dist`:

1. A packed binary will be built via `build-release` and copied into `dist/`
2. Shell completion files and a manpage will also be built and saved into
   `dist/`

## Dependencies

In order to use the full functionality offered by this boilerplate, the
following dependencies must be installed:

* `just bloat`:
  * [cargo-bloat](https://github.com/RazrFalcon/cargo-bloat)
    (`cargo install cargo-bloat`)
* `just build-release`:
  * The toolchain specified by the <code>channel</code> variable.
  * The target specified by the <code>target</code> variable.
  * `strip` (Included with binutils)
  * [`sstrip`](http://www.muppetlabs.com/~breadbox/software/elfkickers.html)
    **(optional)**
  * [`upx`](https://upx.github.io/) (**optional**, `sudo apt-get install upx`)
* `just fmt` and `just fmt-check`:
  * A nightly Rust toolchain
  * (`rustup toolchain install nightly`)
  * [rustfmt](https://github.com/rust-lang/rustfmt) for the nightly toolchain
    (`rustup component add rustfmt --toolchain nightly`)
* `just dist-supplemental`:
   * [help2man](https://www.gnu.org/software/help2man/)
     (`sudo apt-get install help2man`)
* `just kcachegrind`:
   * [Valgrind](http://valgrind.org/) (`sudo apt-get install valgrind`)
   * [KCachegrind](https://kcachegrind.github.io/) (`sudo apt-get install kcachegrind`)
* `just kcov`:
  * A [Rust-compatible build](http://sunjay.ca/2016/07/25/rust-code-coverage) of
kcov
* `just test`:
  * [clippy](https://github.com/rust-lang/rust-clippy)
    (`rustup component add clippy`)
  * [cargo-deadlinks](https://github.com/deadlinks/cargo-deadlinks)
    (`cargo install cargo-deadlinks`)
  * [cargo-outdated](https://github.com/kbknapp/cargo-outdated)
    (`cargo install cargo-outdated`)

### Dependency Installation

* **Debian/Ubuntu/Mint:**

        export PATH="$HOME/.cargo/bin:$PATH"
        cargo install just
        just install-deps

        # ...and now manually install the following optional tools:
        #  - sstrip (from ELFkickers)
        #  - kcov (version 31 or higher with --verify support)

* **Other distros:**

        export PATH="$HOME/.cargo/bin:$PATH"
        cargo install just
        just install-cargo-deps

        # ...and now manually install the following optional tools:
        #  - help2man
        #  - kcachegrind
        #  - kcov (version 31 or higher with --verify support)
        #  - strip (from binutils)
        #  - sstrip (from ELFkickers)
        #  - upx
        #  - valgrind

## TODO

* Get a feel for the workflow surrounding building a project with
  [Failure](https://github.com/rust-lang-nursery/failure) and decide whether to
  rebase this template on top of it.
* Investigate how flexible [QuiCLI](https://github.com/killercup/quicli) and
    its dependency on env_logger are and whether it'd be useful to rebase on it
    or whether I'd just be reinventing most of it anyway to force the exact
    look and feel I achieved with stderrlog.
    (eg. The `Verbosity` struct doesn't implement "`-v` and `-q` are
    mirrors of each other" and I'm rather fond of stderrlog's approach to
    timestamp toggling.)
  * What effect does quicli have on the final binary size? (not a huge concern)
* Investigate why [cargo-cov](https://github.com/kennytm/cov) isn't hiding the
    components of the rust standard library and whether it can be induced to
    generate coverage despite some tests failing. If so, add a command for it.
* Figure out whether StructOpt or Clap is to blame for doubling the leading
  newline when `about` is specified via the doc comment and then report the bug.
* Read the [callgrind docs](http://valgrind.org/docs/manual/cl-manual.html) and
  figure out how to exclude the Rust standard library from what KCacheGrind
  displays.
  * I may need to filter the output.
    [[1]](https://stackoverflow.com/questions/7761448/filter-calls-to-libc-from-valgrinds-callgrind-output)
  * Figure out how to add a `just` task for a faster but less precise profiler
    like [gprof](https://en.wikipedia.org/wiki/Gprof)
    [[1]](http://www.thegeekstuff.com/2012/08/gprof-tutorial/)
    [[2]](https://sourceware.org/binutils/docs/gprof/),
    [OProfile](http://oprofile.sourceforge.net/)
    [[1]](https://llogiq.github.io/2015/07/15/profiling.html), or
    [perf](https://perf.wiki.kernel.org/index.php/Main_Page)
    [[1]](http://blog.adamperry.me/rust/2016/07/24/profiling-rust-perf-flamegraph/)
     to make it easy to leverage the various trade-offs. (And make sure to
     provide convenient access to flame graphs and at least one perf inspector
     GUI or TUI.)
  * Include a reference to [this](http://yosefk.com/blog/how-profilers-lie-the-cases-of-gprof-and-kcachegrind.html)
    blog post on how profilers can can mislead in different ways and probably
    also [this](http://blog.adamperry.me/rust/2016/07/24/profiling-rust-perf-flamegraph/)
    too.
  * Look into options for making it as easy as possible to optimize and
    regression-test runtime performance.
    [[1]](https://github.com/rust-lang/rust/issues/31265https://github.com/rust-lang/rust/issues/31265)
    [[2]](https://crates.io/crates/bencher)
    [[3]](https://github.com/japaric/criterion.rshttps://github.com/japaric/criterion.rs)
    [[4]](https://github.com/BurntSushi/cargo-benchcmp)
* Test and enhance `.travis.yml`
  * Consider officially supporting Windows as a target (probably using
    [cargo-make](https://crates.io/crates/cargo-make) instead of Just) and, if
    I do, come up with an `appveyor.yml`... possibly the one from this project:
    https://github.com/starkat99/appveyor-rust
* Add a `run-memstats` Just task which swaps in jemalloc and sets MALLOC_CONF=stats_print:true
* Investigate commit hooks [[1]](https://stackoverflow.com/questions/3462955/putting-git-hooks-into-repository) [[2]](https://stackoverflow.com/questions/427207/can-git-hook-scripts-be-managed-along-with-the-repository) [[3]](https://mpdaugherty.wordpress.com/2010/04/06/how-to-include-git-hooks-in-a-repository-and-still-personalize-your-machine/)
* Once I've cleared out these TODOs, consider using this space for a reminder
  list of best practices for avoiding "higher-level footguns" noted in my pile
  of assorted advice.
  (Things like "If you can find a way to not need path manipulation beyond
   'pass this opaque token around', then you can eliminate entire classes of
   bugs")
* At least *list* a snip of example code for something like RustyLine as the
  suggested way to do simple user prompting.
* Gather my custom clap validators into a crate, add some more, and have this
  depend on it:
  * Self-Contained data:
    * Boolean is `1`/`y`/`yes`/`t`/`true` or `0`/`n`/`no`/`f`/`false`
      (case-insensitive, include a utility function for actual parsing)
    * Integers:
      * Can be parsed as a decimal integer `> 0` (eg. number of volumes)
      * Can be parsed as a decimal integer `>= 0` (eg. number of bytes)
      * Number of bytes, with optional SI mebi- unit suffix
        (eg. `16m`, including optional `b`, case-insensitive)
    * Floats:
      * Can be parsed as a float in the range `0.0 <= x <= 1.0`
  * Invalidatable/Referential data:
    * Input files:
      * File exists and is readable
      * Directory exists and is browsable (`+rX`)
      * Path is a readable file or browsable directory (ie. read or recurse)
    * Output files:
      * Integers:
        * Augmented "number of bytes, with optional SI mebi- unit suffix"
          validator with upper limit for producing files representable by
          ISO9660/FAT32 filesystems on removable media.
          (2GiB, since some implementations use 32-bit signed offsets)
      * Strings:
        * Is valid FAT32-safe filename/prefix (path separators disallowed)
      * Paths:
        * File path is probably FAT32 writable
          * If file exists, `access()` says it's probably writable
          * If file does not exist, name is FAT32-valid and within a
            probably-writable directory.
        * File path is probably FAT32 writable, with `mkdir -p`
          * Nonexistent path components are FAT32-valid
          * Closest existing ancestor is a probably-writable directory
        * Directory exists and is probably writable
          * "probably writable" is tested via `access()` and will need
            portability shimming.
    * Network I/O:
      * Integers:
        * Successfully parses into a valid listening TCP/UDP port number
          (0-65535, I think)
        * Successfully parses into a valid, non-root, listening TCP/UDP port
          number (0 or 1024-65535, I think)
        * Successfully parses into a valid connecting TCP/UDP port number
          (1-65535, I think)
      * Strings:
        * Successfully parses into a [`SocketAddr`](https://doc.rust-lang.org/std/net/enum.SocketAddr.html) (IP+port, may perform DNS lookup?)
        * Successfully parses into an [`IpAddr`](https://doc.rust-lang.org/std/net/enum.IpAddr.html) (may perform DNS lookup?)
      * URLs:
        * Is well-formed relative URL ([external dependency](https://docs.rs/url/1.4.0/url/) behind a cargo feature)
        * Is well-formed absolute URL ([external dependency](https://docs.rs/url/1.4.0/url/) behind a cargo feature)
