# Version 0.1

# --== Variables to be customized/overridden by the user ==--

export channel = "nightly"
export target = "i686-unknown-linux-musl"
export features = ""

strip_bin = "strip"
strip_flags = "--strip-unneeded"
upx_flags = "--ultra-brute"

# Examples for OpenPandora cross-compilation
# target = "arm-unknown-linux-gnueabi"
# strip_bin = `echo $HOME/opt/pandora-dev/arm-2011.09/bin/pandora-strip`

# --== Code Begins ==--

# Parse the value of the "name" key in the [package] section of Cargo.toml
# using only the commands any POSIX-compliant platform should have
# Source: http://stackoverflow.com/a/40778047/435253
export zz_pkgname=`sed -nr "/^\[package\]/ { :l /^name[ ]*=/ { s/.*=[ ]*//; p; q;}; n; b l;}" Cargo.toml | sed 's@^"\(.*\)"$@\1@'`
export zz_target_path="target/" + target  + "/release/" + zz_pkgname

# `diff`-friendly mapping from `just` to `just test`
DEFAULT: test

# Call `cargo build --release`. Enable size optimizations if `channel=nightly`.
build:
	#!/bin/sh
	# If on nightly, opt-level=z and alloc_system to shrink output further
	if [ "$channel" = "nightly" ]; then
		features="nightly $features"
		# TODO: Find a less hacky way to do this
		cleanup() {
			sed -i 's/opt-level = "z"/opt-level = 3/' Cargo.toml
		}
		trap cleanup EXIT
		sed -i 's/opt-level = 3/opt-level = "z"/' Cargo.toml
	fi
	printf "\n--== Building with %s for %s (features: %s) ==--\n" "$channel" "$target" "$features"
	cargo "+$channel" build --release --target="$target" "--features=$features"

# Call `build` and then strip and compress the resulting binary
build-release: miniclean build
	@# Depend on miniclean since stripping UPXd executables is fatal
	@printf "\n--== Stripping, SStripping, and Compressing With UPX ==--\n"
	{{strip_bin}} {{strip_flags}} "{{zz_target_path}}"
	@# Allow sstrip to fail because it can't be installed via "just install-deps"
	sstrip "{{zz_target_path}}" || true
	upx {{upx_flags}} "{{zz_target_path}}"
	@printf "\n--== Final Result ==--\n"
	@ls -sh "{{zz_target_path}}"
	@printf "\n"

# Alias for `cargo fmt -- {{args}}`
fmt +args="":
	cargo fmt -- {{args}}

# Ensure `strip` and `upx` are installed via `apt-get`.
install-apt-deps:
	sudo apt-get install binutils upx

# `install-rustup-deps` and then `cargo install` tools
install-cargo-deps: install-rustup-deps
	@# Prevent "already installed" from causing a failure
	cargo install rustfmt || true
	cargo install cargo-deadlinks || true
	cargo install cargo-outdated || true
	cargo +nightly install clippy || true

# Install (but don't update) nightly, stable, and `channel` toolchains, plus `target`.
install-rustup-deps:
	@# Prevent this from gleefully doing an unwanted "rustup update"
	rustup toolchain list | grep -q stable || rustup toolchain install stable
	rustup toolchain list | grep -q nightly || rustup toolchain install nightly
	rustup toolchain list | grep -q '{{channel}}' || rustup toolchain install '{{channel}}'
	rustup target list | grep -q '{{target}} (' || rustup target add '{{target}}'

# Run `install-apt-deps` and `install-cargo-deps`, then list what remains.
@install-deps: install-apt-deps install-cargo-deps
	echo
	echo "-----------------------------------------------------------"
	echo "IMPORTANT: You will need to install the following manually:"
	echo "-----------------------------------------------------------"
	echo " * Rust-compatible kcov (http://sunjay.ca/2016/07/25/rust-code-coverage)"
	echo " * sstrip (http://www.muppetlabs.com/%7Ebreadbox/software/elfkickers.html)"

# Generate a statement coverage report in `target/cov/`
kcov:
	#!/bin/bash
	# Sources:
	# - http://sunjay.ca/2016/07/25/rust-code-coverage
	# - https://users.rust-lang.org/t/tutorial-how-to-collect-test-coverages-for-rust-project/650
	#
	# TODO:
	# - Try to replace the array operations that are keeping this from /bin/sh
	# - https://gist.github.com/colin-kiegel/e3a1fea04cd3ad8ed06d
	# - https://github.com/rust-lang/cargo/issues/1924#issuecomment-198648663
	# - https://gist.github.com/dikaiosune/07177baf5cea76c27783efa55e99da89
	#
	# As of July 2, 2016, there is no option to make rustdoc generate a runnable
	# test executable. That means that documentation tests will not show in your
	# coverage data. If you discover a way to run the doctest executable with kcov,
	# please open an Issue and we will add that to these instructions.
	# -- https://github.com/codecov/example-rust
	PKGID="$(cargo pkgid)"
	[ -z "$PKGID" ] && exit 1
	ORIGIN="${PKGID%#*}"
	ORIGIN="${ORIGIN#file://*}"

	# Ensure that kcov can see totally unused functions
	# Sources:
	# - http://stackoverflow.com/a/38371687/435253
	# - https://gist.github.com/dikaiosune/07177baf5cea76c27783efa55e99da89
	export RUSTFLAGS='-C link-dead-code'
	cargo clean -v

	shift
	cargo test --no-run || exit $?
	EXE=($ORIGIN/target/debug/$zz_pkgname-*)
	if [ ${#EXE[@]} -ne 1 ]; then
		echo 'Non-unique test file, retrying...' >2
		rm -f "${EXE[@]}"
		cargo test --no-run || exit $?
	fi
	rm -rf "$ORIGIN/target/cov"
	kcov --exclude-pattern=/.cargo,/usr/lib --verify "$ORIGIN/target/cov" "$ORIGIN/target/debug/$zz_pkgname-"* "$@"

# Remove the release binary. (Used to avoid `strip`-ing UPX'd files.)
@miniclean:
	rm -f "{{zz_target_path}}"

# Alias for `cargo fmt -- {{args}}` with the *default* toolchain
run +args="":
	cargo run -- {{args}}

# Run all installed static analysis, plus `cargo +stable test`.
test:
	@echo "--== Coding Style ==--"
	cargo fmt -- --write-mode checkstyle | grep -v '<'
	@echo "--== Outdated Packages ==--"
	cargo outdated
	@printf "\n--== Dead Internal Documentation Links ==--\n"
	cargo doc && cargo deadlinks
	@printf "\n--== Clippy Lints ==--\n"
	cargo +nightly clippy  # Run clippy for maximum pedantry
	@printf "\n--== Test Suite (on stable) ==--\n"
	cargo +stable test  # Test with stable so nightly dependencies don't slip in

	# TODO: https://users.rust-lang.org/t/howto-sanitize-your-rust-code/9378
	#	  (And use clippy as a compiler plugin so we can save a pass)


# vim: set ft=make textwidth=100 colorcolumn=101 noexpandtab sw=8 sts=8 ts=8 :
