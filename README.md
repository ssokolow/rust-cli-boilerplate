# Rust CLI Project Template

The base project template I use with
[cargo-template](https://github.com/pwoolcoc/cargo-template/) for starting
new projects in the [Rust](https://rust-lang.org/) programming language.

Given the current lack of mature equivalents to
[Django](https://www.djangoproject.com/) and
[PyQt](https://riverbankcomputing.com/news) (which run on stable-channel Rust),
this template is primarily optimized for building command-line tools.

I'll probably build another one for use with
[rust-cypthon](https://github.com/dgrunwald/rust-cpython) later.

## Application Boilerplate Features

* Uses [clap](https://clap.rs/) for argument parsing
* Uses [error-chain](https://github.com/brson/error-chain) for unified error
  handling
* Opts into almost all available rustc and
  [clippy](https://github.com/Manishearth/rust-clippy) lints without requiring
  that all builds be done on a clippy-enabled Rust version.
* `release.sh --nightly` produces a fully statically-linked i686 binary where
  the base size of the boilerplate, including statically-linked musl-libc,
  error-chain, and "Did you mean...?"-enabled clap, is 178K.

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
  <td>A whitelist for CamelCase names which produce false positives in Clippy's
"identifier needs backticks" lint.</td>
</tr>
<tr>
  <td><code>rustfmt.toml</code></td>
  <td>A definition of my preferred coding style</td>
</tr>
<tr><th colspan="2">Helper Scripts</th></tr>
<tr>
  <td><code>coverage.sh</code></td>
  <td>Automation for generating a statement coverage report at <code>target/cov/</code>. (Branch coverage is pending something less involved than <a href="https://users.rust-lang.org/t/howto-generating-a-branch-coverage-report/8524">this</a>.)</td>
</tr>
<tr>
  <td><code>release.sh</code></td>
  <td>Build automation to produce the most compact statically-linked binary
possible. (A workaround for the inability to define a post-build script for <code>cargo build --release</code>)</td>
</tr>
<tr>
  <td><code>test.sh</code></td>
  <td>A script to run the most pedantic run of automated testing and static analysis that is feasible.</td>
</tr>
</table>
</html>

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

### If built via `./release.sh`:

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

### If built via `./release.sh --nightly`:

1. Unless otherwise noted, all optimizations listed above.
2. The binary will be built with `opt-level = "z"` to further reduce file size.
3. The binary will be built against the system memory allocator to avoid the
   overhead of bundling a copy of jemalloc.

## Dependencies

In order to use the full functionality offered by this boilerplate, the
following system-level dependencies must be installed:

* `coverage.sh`:

  * A [Rust-compatible build](http://sunjay.ca/2016/07/25/rust-code-coverage) of
kcov

* `release.sh`:

  * 32-bit musl-libc targeting support
    (`rustup target add i686-unknown-linux-musl`)
  * `strip` (Included with binutils)
  * [`sstrip`](http://www.muppetlabs.com/~breadbox/software/elfkickers.html)
  * [`upx`](https://upx.github.io/) (`sudo apt-get install upx`)

* `release.sh --nightly`:

  * The base requirements for `release.sh`
  * A nightly Rust toolchain (`rustup toolchain add nightly`)

* `test.sh`:

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

**Note:** `release.sh` also contains commented-out lines to enable targeting
the [OpenPandora](http://openpandora.org/) Linux palmtop using a
[cross-compiling gcc toolchain](https://pandorawiki.org/Cross-compiler) for
the final glibc link.

## TODO

* Set up [slog](https://github.com/slog-rs/slog) [[1]](https://docs.rs/slog-scope/0.2.2/slog_scope/) as an analogue to the
  `logging` module from Python stdlib
* Consider using [just](https://github.com/casey/just) for helper scripting
* Add a table to "Build Behaviour" showing output file sizes
* Gather my custom clap validators into a crate and have this depend on it:

  * Can be parsed as an integer > 0 (eg. number of volumes)
  * Can be parsed as an integer >= 0 (eg. number of bytes)
  * File path exists and is readable
  * Target directory probably writable (via `access()`)
  * Filename/path contains no characters that are invalid on FAT32 thumbdrives

* Add ready-to-run CI boilerplate, such as a `.travis.yml`
* Check what effect 32-bit vs. 64-bit musl targeting has on UPXed file size,
  if any.
* Investigate commit hooks
