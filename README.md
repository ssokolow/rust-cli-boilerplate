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

* Uses [clap](https://clap.rs/) (with "Did you mean...?" suggestions enabled)
  for argument parsing.
* Uses [error-chain](https://github.com/brson/error-chain) for unified error
  handling
* Enables almost all rustc and
  [clippy](https://github.com/Manishearth/rust-clippy) lints without making
  clippy mandatory.
* Takes advantage of nightly-only features without requiring nightly.
* A comprehensive set of [just](https://github.com/casey/just) commands, easily
  customized via variables. (eg. for cross-compilation)
* `just build-release` for a 100% static i686 binary totalling under `205KiB`
  (`185KiB` with `panic="abort"`) in new projects
* `just install-deps` to install all but two optional dependencies on
  Debian-family distros.
* `just install-cargo-deps` to install all distro-agnostic dependencies.

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
  <td><code>channel</code></td>
  <td><code>nightly</code></code></td>
  <td><code>rustc</code> channel used for <code>build</code> and dependent commands.</td>
</tr>
<tr>
  <td><code>target</code></td>
  <td><code>i686-unknown-linux-musl</code></td>
  <td>Used for <code>build</code> and additionally installed by <code>install-rustup-deps</code></td>
</tr>
<tr>
  <td><code>features</code></td>
  <td></td>
  <td>Extra features to enable. Gains <code>nightly</code> when <code>channel=nightly</code></td>
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
  <td>Flags passed to UPX.</td>
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
  <td><code>build</code></td>
  <td></td>
  <td>Call <code>cargo build --release</code>. Enable size optimizations if <code>channel=nightly</code></td>
</tr>
<tr>
  <td><code>build-release</code></td>
  <td></td>
  <td>Call <code>build</code> and then strip and compress the resulting binary.</td>
</tr>
<tr>
  <td><code>fmt</code></td>
  <td>args (optional)</td>
  <td>Alias for <code>cargo fmt -- {{args}}</code></td>
</tr>
<tr>
  <td><code>install-apt-deps</code></td>
  <td></td>
  <td>Ensure <code>strip</code> and <code>upx</code> are installed via
<code>apt-get</code></td>
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
  <td>Install (but don't update) nightly, stable, and <code>channel</code>
toolchains, plus <code>target</code></td>
</tr>
<tr>
  <td><code>install-deps</code></td>
  <td></td>
  <td>Run <code>install-apt-deps</code> and <code>install-cargo-deps</code>,
then list what remains.</td>
</tr>
<tr>
  <td><code>kcov</code></td>
  <td></td>
  <td>Generate a statement coverage report in <code>target/cov/</code></td>
</tr>
<tr>
  <td><code>miniclean</code></td>
  <td></td>
  <td>Remove the release binary. (Used to avoid <code>strip</code>-ing UPX'd files.)</td>
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
3. Panic via `abort` rather than unwinding to allow backtrace code to be pruned
   away by dead code optimization.

### If built via `just channel=stable build-release`:

1. Unless otherwise noted, all optimizations listed above.
2. The binary will be statically linked against
   [musl-libc](http://www.musl-libc.org/) for maximum portability.
3. The binary will be stripped with `--strip-unneeded` and then with
   [`sstrip`](http://www.muppetlabs.com/~breadbox/software/elfkickers.html)
   (a more aggressive companion used in embedded development) to produce the
   smallest possible pre-compression size.
4. The binary will be compressed via
   [`upx --ultra-brute`](https://upx.github.io/).
   In my experience, this makes a file about 1/3rd the size of the input.

### If built via `just channel=nightly build-release`:

1. Unless otherwise noted, all optimizations listed above.
2. The binary will be built with `opt-level = "z"` to further reduce file size.
3. The binary will be built against the system memory allocator to avoid the
   overhead of bundling a copy of jemalloc.

## Dependencies

In order to use the full functionality offered by this boilerplate, the
following dependencies must be installed:

* `just build-release`:

  * The toolchain specified by the <code>channel</code> variable.
  * 32-bit musl-libc targeting support
    (`rustup target add i686-unknown-linux-musl`)
  * `strip` (Included with binutils)
  * [`sstrip`](http://www.muppetlabs.com/~breadbox/software/elfkickers.html)
    **(optional)**
  * [`upx`](https://upx.github.io/) (`sudo apt-get install upx`)
* `just kcov`:

  * A [Rust-compatible build](http://sunjay.ca/2016/07/25/rust-code-coverage) of
kcov

* `just test`:

  * A stable Rust toolchain (`rustup toolchain add stable`)
  * A nightly Rust toolchain (`rustup toolchain add nightly`)
  * [cargo-deadlinks](https://github.com/deadlinks/cargo-deadlinks)
    (`cargo install cargo-deadlinks`)
  * [cargo-outdated](https://github.com/kbknapp/cargo-outdated)
    (`cargo install cargo-outdated`)
  * [clippy](https://github.com/Manishearth/rust-clippy)
    (`cargo +nightly install clippy`)
  * [rustfmt](https://github.com/rust-lang-nursery/rustfmt)
    (`cargo install rustfmt`)

### Dependency Installation

* **Debian/Ubuntu/Mint:**

        export PATH="$HOME/.cargo/bin:$PATH"
        cargo install just
        just install-deps

* **Other distros:**

        export PATH="$HOME/.cargo/bin:$PATH"
        cargo install just
        just install-cargo-deps
        # ...and now manually make sure `strip` and `upx` are installed

## TODO

* Set up [slog](https://github.com/slog-rs/slog) [[1]](https://docs.rs/slog-scope/0.2.2/slog_scope/) as an analogue to the
  `logging` module from Python stdlib
* Add ready-to-run CI boilerplate, such as a `.travis.yml`
* Investigate commit hooks
* Gather my custom clap validators into a crate and have this depend on it:

  * Can be parsed as an integer > 0 (eg. number of volumes)
  * Can be parsed as an integer >= 0 (eg. number of bytes)
  * File path exists and is readable
  * Target directory probably writable (via `access()`)
  * Filename/path contains no characters that are invalid on FAT32 thumbdrives
