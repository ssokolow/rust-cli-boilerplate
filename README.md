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
* Enables almost all rustc and
  [clippy](https://github.com/rust-lang/rust-clippy) lints without making
  clippy mandatory.
* A comprehensive set of [just](https://github.com/casey/just) commands, easily
  customized via variables. (eg. for cross-compilation)
* `just build-release` for a 100% static i686 binary totalling roughly `236KiB`
  (`212KiB` with `panic="abort"`) in new projects
* `just install-deps` to install all but two optional dependencies on
  Debian-family distros.
* `just install-cargo-deps` to install all distro-agnostic dependencies.
* A basic `.travis.yml` for use with [Travis-CI](https://travis-ci.org/) and
  [Nightli.es](https://nightli.es/).

## Supplementary Files

<html>
<table>
<tr><th colspan="2">Metadata</th></tr>
<tr>
  <td><code>LICENSE</code></td>
  <td>A copy of the <a href="https://www.gnu.org/licenses/gpl-3.0.html">GNU GPLv3</a> as my "until I've had time to think about it"
license of choice.</td>
</tr>
<tr><th colspan="2">Configuration</th></tr>
<tr>
  <td><code>.gitignore</code></td>
  <td>Just ignore <code>/target</code> since that's where Cargo puts everything.</td>
</tr>
<tr>
  <td><code>clippy.toml</code></td>
  <td>Whitelist for CamelCase names which trigger Clippy's "identifier needs
  backticks" lint.</td>
</tr>
<tr>
  <td><code>rustfmt.toml</code></td>
  <td>Show TODO/FIXME comments and tweak default <code>rustfmt</code> style</td>
</tr>
<tr><th colspan="2">Development Automation</th></tr>
<tr>
  <td><code>justfile</code></td>
  <td>Build/development-automation commands via <a href="https://github.com/casey/just">just</a> (a pure-Rust make-alike).</td>
</tr>
</table>
</html>

## Justfile Reference

### Variables (`just --evaluate`)
<html>
<table>
<tr><th>Variable</th><th>Default Value</th><th>Description</th></tr>
<tr>
  <td><code>callgrind_args</code></td>
  <td></td>
  <td>Extra arguments to pass to <a href="http://valgrind.org/docs/manual/cl-manual.html">callgrind</a>.</td>
</tr>
<tr>
  <td><code>callgrind_out_file</code></td>
  <td><code>callgrind.out.justfile</code></td>
  <td>Temporary file used by <code>just kcachegrind</code></td>
</tr>
<tr>
  <td><code>channel</code></td>
  <td><code>stable</code></code></td>
  <td><code>rustc</code> channel used for <code>build</code> and dependent commands.</td>
</tr>
<tr>
  <td><code>features</code></td>
  <td></td>
  <td>Extra features to enable.</td>
</tr>
<tr>
  <td><code>target</code></td>
  <td><code>i686-unknown-linux-musl</code></td>
  <td>Used for <code>build</code> and additionally installed by <code>install-rustup-deps</code></td>
</tr>
<tr>
  <td><code>strip_bin</code></td>
  <td><code>strip</code></td>
  <td>Override when cross-compiling. See <code>justfile</code> source for example.</td>
</tr>
<tr>
  <td><code>strip_flags</code></td>
  <td><code>--strip-unneeded</code></td>
  <td>Flags passed to <code>strip_bin</code></td>
</tr>
<tr>
  <td><code>upx_flags</code></td>
  <td><code>--ultra-brute</code></td>
  <td>Flags passed to <a href="https://upx.github.io/">UPX</a>.</td>
</tr>
</table>
</html>

### Commands (`just --list`)
<html>
<table>
<tr><th>Command</th><th>Arguments</th><th>Description</th></tr>
<tr>
  <td><code>DEFAULT</code></td>
  <td></td>
  <td><code>diff</code>-friendly mapping from <code>just</code> to <code>just
test</code></td>
</tr>
<tr>
  <td><code>bloat</code></td>
  <td>args (optional)</td>
  <td>Call <code>cargo bloat --release</code></td>
<tr>
  <td><code>build</code></td>
  <td></td>
  <td>Call <code>cargo build --release</code></td>
</tr>
<tr>
  <td><code>build-release</code></td>
  <td></td>
  <td>Call <code>build</code> and then strip and compress the resulting binary</td>
</tr>
<tr>
  <td><code>check</code></td>
  <td>args (optional)</td>
  <td>Alias for <code>cargo check {{args}}</code> with the default toolchain</code></td>
</tr>
<tr>
  <td><code>clean</code></td>
  <td>args (optional)</td>
  <td>Alias for <code>cargo clean -v {{args}}</code> with the default toolchain</code></td>
</tr>
<tr>
  <td><code>doc</code></td>
  <td>args (optional)</td>
  <td>Alias for <code>cargo doc --document-private-items {{args}}</code></td>
</tr>
<tr>
  <td><code>fmt</code></td>
  <td>args (optional)</td>
  <td>Alias for <code>cargo fmt -- {{args}}</code></td>
</tr>
<tr>
  <td><code>install-apt-deps</code></td>
  <td></td>
  <td>Use <code>apt-get</code> to install dependencies <code>cargo</code> can't
(except <code>kcov</code> and <code>sstrip</code>)</td>
</tr>
<tr>
  <td><code>install-cargo-deps</code></td>
  <td></td>
  <td><code>install-rustup-deps</code> and then <code>cargo install</code>
tools.</td>
</tr>
<tr>
  <td><code>install-rustup-deps</code></td>
  <td></td>
  <td>Install (don't update) nightly, stable, and <code>channel</code>
toolchains, plus <code>target</code>, clippy, and rustfmt</td>
</tr>
<tr>
  <td><code>install-deps</code></td>
  <td></td>
  <td>Run <code>install-apt-deps</code> and <code>install-cargo-deps</code>,
list what remains.</td>
</tr>
<tr>
  <td><code>kcachegrind</code></td>
  <td></td>
  <td>Run a debug build under
  <a href="http://valgrind.org/docs/manual/cl-manual.html">callgrind</a>, then
  open the profile in
  <a href="https://kcachegrind.github.io/">KCachegrind</a>.</td>
</tr>
<tr>
  <td><code>kcov</code></td>
  <td></td>
  <td>Generate a statement coverage report in <code>target/cov/</code></td>
</tr>
<tr>
  <td><code>run</code></td>
  <td>args (optional)</td>
  <td>Alias for <code>cargo run -- {{args}}</code> with the <em>default</em>
toolchain.</td>
</tr>
<tr>
  <td><code>test</code></td>
  <td></td>
  <td>Run all installed static analysis, plus <code>cargo +stable
test</code></td>
</tr>
</table>
</html>


### Tips

* Edit the `DEFAULT` command. That's what it's there for.
* You can use `just` from any subdirectory in your project. It's like `git` that way.
* `just path/to/project/` (note the trailing slash) is equivalent to `(cd path/to/project; just)`
* `just path/to/project/command` is equivalent to `(cd path/to/project; just command)`

## Build Behaviour

In order to be as suitable as possible for building self-contained,
high-reliability replacements for shell scripts, the following build options
are defined:

### If built via `cargo build`:

1. Backtrace support will be disabled in `error-chain` unless explicitly
   built with the `backtrace` feature. (This began as a workaround to unbreak
   cross-compiling to musl-libc and ARM after backtrace-rs 0.1.6 broke it, but
   it also makes sense to opt out of it if I'm using `panic="abort"` to save
   space)

### If built via `cargo build --release`:

1. Unless otherwise noted, all optimizations listed above.
2. Link-time optimization will be enabled (`lto = true`)
3. Optionally (uncomment a line in `Cargo.toml`) panic via `abort` rather than
   unwinding to allow backtrace code to be pruned away by dead code
   optimization.

### If built via `just build-release`:

1. Unless otherwise noted, all optimizations listed above.
2. The binary will be built with `opt-level = "z"` to further reduce file size.
3. The binary will be statically linked against
   [musl-libc](http://www.musl-libc.org/) for maximum portability.
4. The binary will be stripped with `--strip-unneeded` and then with
   [`sstrip`](http://www.muppetlabs.com/~breadbox/software/elfkickers.html)
   (a more aggressive companion used in embedded development) to produce the
   smallest possible pre-compression size.
5. The binary will be compressed via
   [`upx --ultra-brute`](https://upx.github.io/).
   In my experience, this makes a file about 1/3rd the size of the input.

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
  * [`upx`](https://upx.github.io/) (`sudo apt-get install upx`)
* `just kcachegrind`:
   * [Valgrind](http://valgrind.org/) (`sudo apt-get install valgrind`)
   * [KCachegrind](https://kcachegrind.github.io/) (`sudo apt-get install kcachegrind`)
* `just kcov`:
  * A [Rust-compatible build](http://sunjay.ca/2016/07/25/rust-code-coverage) of
kcov
* `just test`:
  * A stable Rust toolchain (`rustup toolchain add stable`)
  * [clippy](https://github.com/rust-lang/rust-clippy)
    (`rustup component add clippy`)
  * [rustfmt](https://github.com/rust-lang/rustfmt)
    (`rustup component add rustfmt`)
  * [cargo-deadlinks](https://github.com/deadlinks/cargo-deadlinks)
    (`cargo install cargo-deadlinks`)
  * [cargo-outdated](https://github.com/kbknapp/cargo-outdated)
    (`cargo install cargo-outdated`)

### Dependency Installation

* **Debian/Ubuntu/Mint:**

        export PATH="$HOME/.cargo/bin:$PATH"
        cargo install just
        just install-deps

        # ...and now  manually make sure the following tools are installed:
        #  - sstrip (optional, from ELFkickers)
        #  - kcov (optional, version 31 or higher with --verify support)

* **Other distros:**

        export PATH="$HOME/.cargo/bin:$PATH"
        cargo install just
        just install-cargo-deps

        # ...and now manually make sure the following tools are installed:
        #  - strip (from binutils)
        #  - upx
        #  - sstrip (optional, from ELFkickers)
        #  - kcachegrind (optional)
        #  - kcov (optional, version 31 or higher with --verify support)
        #  - valgrind (optional)

## TODO

* Update this for a modern `Cargo.toml` template renderer.
  * Use a `year` template variable to automatically fill out copyright dates.
* Re-incorporate use of `nightly` for access to unstable rustfmt options.
* Add [log](https://github.com/rust-lang-nursery/log) to the boilerplate
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
* Investigate commit hooks [[1]](https://stackoverflow.com/questions/3462955/putting-git-hooks-into-repository) [[2]](https://stackoverflow.com/questions/427207/can-git-hook-scripts-be-managed-along-with-the-repository) [[3]](https://mpdaugherty.wordpress.com/2010/04/06/how-to-include-git-hooks-in-a-repository-and-still-personalize-your-machine/)
* Once I've cleared out these TODOs, consider using this space for a reminder
  list of best practices for avoiding "higher-level footguns" noted in my pile
  of assorted advice.
  (Things like "If you can find a way to not need path manipulation beyond
   'pass this opaque token around', then you can eliminate entire classes of
   bugs")
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
