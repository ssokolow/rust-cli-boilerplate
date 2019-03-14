# Version 0.2
# Copyright 2017-2019, Stephan Sokolow

# --== Variables to be customized/overridden by the user ==--

export CARGO_BUILD_TARGET = "i686-unknown-linux-musl"

channel = "stable"
features = ""

build_flags = "--release"
strip_bin = "strip"
strip_flags = "--strip-unneeded"
sstrip_bin = "sstrip"
upx_flags = "--ultra-brute"
callgrind_args = ""
callgrind_out_file = "callgrind.out.justfile"

bash_completion_dir = "~/.bash_completion.d"
fish_completion_dir = "~/.config/fish/completions"
zsh_completion_dir = "~/.zsh/functions"
manpage_dir = "~/.cargo/share/man/man1"

# Examples for OpenPandora cross-compilation
# export CARGO_BUILD_TARGET = "arm-unknown-linux-gnueabi"
# strip_bin = `echo $HOME/opt/pandora-dev/arm-2011.09/bin/pandora-strip`

# --== Code Begins ==--

# Internal variables
# TODO: Look up that GitHub issues post on whitespace handling
_cargo = "cargo \"+" + channel + "\""
_build_flags = "--features=\"" + features + "\" " + build_flags

# Parse the value of the "name" key in the [package] section of Cargo.toml
# using only the commands any POSIX-compliant platform should have
# Source: http://stackoverflow.com/a/40778047/435253
export _pkgname=`sed -nr "/^\[package\]/ { :l /^name[ ]*=/ { s/.*=[ ]*//; p; q;}; n; b l;}" Cargo.toml | sed 's@^"\(.*\)"$@\1@'`
export _target_path="target/" + CARGO_BUILD_TARGET  + "/release/" + _pkgname

# Shorthand for `just test`
DEFAULT: test

# Alias for `cargo bloat`
bloat +args="":
	{{_cargo}} bloat {{_build_flags}} {{args}}

# Alias for `cargo build`
build:
	@echo "\n--== Building with {{channel}} for {{CARGO_BUILD_TARGET}} (features: {{features}}) ==--\n"
	{{_cargo}} build {{_build_flags}}

# Call `build` and then strip and compress the resulting binary
build-release: build
	@# Don't modify the original "cargo" output. That confuses cargo somehow.
	cp "{{_target_path}}" "{{_target_path}}.stripped"
	@printf "\n--== Stripping, SStripping, and Compressing With UPX ==--\n"
	{{strip_bin}} {{strip_flags}} "{{_target_path}}.stripped"
	@# Allow sstrip to fail because it can't be installed via "just install-deps"
	{{sstrip_bin}} "{{_target_path}}.stripped" || true
	@# Allow upx to fail in case the user wants to force no UPXing by leaving it uninstalled
	cp "{{_target_path}}.stripped" "{{_target_path}}.packed"
	upx {{upx_flags}} "{{_target_path}}.packed" || true
	@# Display the resulting file sizes so we can keep an eye on them
	@# (Separate `ls` invocations are used to force the display ordering)
	@printf "\n--== Final Result ==--\n"
	@ls -1sh "{{_target_path}}"
	@ls -1sh "{{_target_path}}.stripped"
	@ls -1sh "{{_target_path}}.packed"
	@printf "\n"

# Alias for `cargo check`
check +args="":
	{{_cargo}} check {{_build_flags}} {{args}}

# Superset of `cargo clean -v` which deletes other stuff this justfile builds
clean +args="":
	{{_cargo}} clean -v {{args}}
	export CARGO_TARGET_DIR="target/kcov" && {{_cargo}} clean -v
	rm -rf dist
# Build the shell completions and a help file, and put them in a "dist" folder
dist-supplemental:
	mkdir -p dist
	@# Generate completions and store them in dist/
	{{_cargo}} run {{_build_flags}} -- --dump-completions bash > dist/{{ _pkgname }}.bash
	{{_cargo}} run {{_build_flags}} -- --dump-completions zsh > dist/{{ _pkgname }}.zsh
	{{_cargo}} run {{_build_flags}} -- --dump-completions fish > dist/{{ _pkgname }}.fish
	{{_cargo}} run {{_build_flags}} -- --dump-completions elvish > dist/{{ _pkgname }}.elvish
	{{_cargo}} run {{_build_flags}} -- --dump-completions powershell > dist/{{ _pkgname }}.powershell
	@# Generate manpage and store it gzipped in dist/
	@# (This comes last so the earlier calls to `cargo run` will get the compiler warnings out)
	help2man -N '{{_cargo}} run {{_build_flags}} -- --help' \
		| gzip -9 > dist/{{ _pkgname }}.1.gz || true

# Call `dist` and `build-release` and copy the packed binary to dist/
dist: build-release dist-supplemental
	@# Copy the packed command to dist/
	cp  "{{ _target_path }}.packed" dist/{{ _pkgname }}

# Run rustdoc with `--document-private-items` and then run cargo-deadlinks
doc +args="":
	{{_cargo}} doc --document-private-items {{_build_flags}} {{args}} && \
	{{_cargo}} deadlinks --dir target/$CARGO_BUILD_TARGET/doc/{{_pkgname}}

# Alias for `cargo +nightly fmt -- {{args}}`
fmt +args="":
	cargo +nightly fmt -- {{args}}

# Alias for `cargo +nightly fmt -- --check {{args}} which un-bloats TODO/FIXME warnings
fmt-check +args="":
	cargo +nightly fmt -- --check --color always {{args}} 2>&1 | egrep -v '[0-9]*[ ]*\|'

# Install the binary, shell completions, and a help file
install: dist-supplemental
	@# Install completions
	@# NOTE: bash and zsh completion requires additional setup to source a non-root dir
	mkdir -p {{bash_completion_dir}} {{zsh_completion_dir}} {{ fish_completion_dir }} {{ manpage_dir }}
	cp dist/{{ _pkgname }}.bash {{ bash_completion_dir }}/{{ _pkgname }}
	cp dist/{{ _pkgname }}.zsh {{ zsh_completion_dir }}/_{{ _pkgname }}
	cp dist/{{ _pkgname }}.fish {{ fish_completion_dir }}/{{ _pkgname }}.fish
	@# Install the manpage
	cp dist/{{ _pkgname }}.1.gz {{ manpage_dir }}/{{ _pkgname }}.1.gz || true
	@# Install the command to ~/.cargo/bin
	{{_cargo}} install --path . --force --features="{{features}}"

# Remove any files installed by the `install` task (but leave any parent directories created)
uninstall:
	@# TODO: Implement the proper fallback chain from `cargo install`
	rm ~/.cargo/bin/{{ _pkgname }} || true
	rm {{ manpage_dir }}/{{ _pkgname }}.1.gz || true
	rm {{ bash_completion_dir }}/{{ _pkgname }} || true
	rm {{ fish_completion_dir }}/{{ _pkgname }}.fish || true
	rm {{ zsh_completion_dir }}/_{{ _pkgname }} || true

# Use `apt-get` to install dependencies `cargo` can't (except `kcov` and `sstrip`)
install-apt-deps:
	sudo apt-get install binutils help2man kcachegrind upx valgrind

# `install-rustup-deps` and then `cargo install` tools
install-cargo-deps: install-rustup-deps
	@# Prevent "already installed" from causing a failure
	cargo install cargo-deadlinks || true
	cargo install cargo-bloat || true
	cargo install cargo-outdated || true

# Install (don't update) nightly `channel` toolchains, plus `CARGO_BUILD_TARGET`, clippy, and rustfmt
install-rustup-deps:
	@# Prevent this from gleefully doing an unwanted "rustup update"
	rustup toolchain list | grep -q '{{channel}}' || rustup toolchain install '{{channel}}'
	rustup toolchain list | grep -q nightly || rustup toolchain install nightly
	rustup target list | grep -q '{{CARGO_BUILD_TARGET}} (' || rustup target add '{{CARGO_BUILD_TARGET}}'
	rustup component list | grep -q 'clippy-\S* (' || rustup component add clippy
	rustup component list --toolchain nightly | grep 'rustfmt-\S* (' || rustup component add rustfmt --toolchain nightly

# Run `install-apt-deps` and `install-cargo-deps`, list what remains.
@install-deps: install-apt-deps install-cargo-deps
	echo
	echo "-----------------------------------------------------------"
	echo "IMPORTANT: You will need to install the following manually:"
	echo "-----------------------------------------------------------"
	echo " * Rust-compatible kcov (http://sunjay.ca/2016/07/25/rust-code-coverage)"
	echo " * sstrip (http://www.muppetlabs.com/%7Ebreadbox/software/elfkickers.html)"

# Run a debug build under callgrind, then open the profile in KCachegrind.
kcachegrind +args="":
	{{_cargo}} build
	rm -rf '{{ callgrind_out_file }}'
	valgrind --tool=callgrind --callgrind-out-file='{{ callgrind_out_file }}' {{ callgrind_args }} 'target/debug/{{ _pkgname }}' '{{ args }}' || true
	test -e '{{ callgrind_out_file }}'
	kcachegrind '{{ callgrind_out_file }}'

# Generate a statement coverage report in `target/cov/`
kcov:
	#!/bin/sh
	# Adapted from:
	# - http://sunjay.ca/2016/07/25/rust-code-coverage
	# - https://users.rust-lang.org/t/tutorial-how-to-collect-test-coverages-for-rust-project/650
	#
	# As of July 2, 2016, there is no option to make rustdoc generate a runnable
	# test executable. That means that documentation tests will not show in your
	# coverage data. If you discover a way to run the doctest executable with kcov,
	# please open an Issue and we will add that to these instructions.
	# -- https://github.com/codecov/example-rust

	# Ensure that kcov can see totally unused functions without clobbering regular builds
	# Adapted from:
	# - http://stackoverflow.com/a/38371687/435253
	# - https://gist.github.com/dikaiosune/07177baf5cea76c27783efa55e99da89
	export CARGO_TARGET_DIR="target/kcov"
	export RUSTFLAGS='-C link-dead-code'
	kcov_path="$CARGO_TARGET_DIR/html"

	if [ "$#" -gt 0 ]; then shift; fi # workaround for "can't shift that many" being fatal in dash
	cargo test --no-run || exit $?
	rm -rf "$kcov_path"

	for file in "$CARGO_TARGET_DIR"/"$CARGO_BUILD_TARGET"/debug/$_pkgname-*; do
		if [ -x "$file" ]; then
			outpath="$kcov_path/$(basename "$file")"
			mkdir -p "$outpath"
			kcov --exclude-pattern=/.cargo,/usr/lib --verify "$outpath" "$file" "$@"
		elif echo "$file" | grep -F -e '-*'; then
			echo "No build files found for coverage!"
			exit 1
		fi
	done

# Alias for `cargo run -- {{args}}`
run +args="":
	{{_cargo}} run {{_build_flags}} -- {{args}}

# Run all installed static analysis, plus `cargo test`.
test:
	@echo "--== Outdated Packages ==--"
	{{_cargo}} outdated
	@printf "\n--== Clippy Lints ==--\n"
	{{_cargo}} clippy {{_build_flags}}
	@printf "\n--== Dead Internal Documentation Links ==--\n"
	{{_cargo}} doc --document-private-items {{_build_flags}} && \
	{{_cargo}} deadlinks --dir target/$CARGO_BUILD_TARGET/doc/{{_pkgname}}
	@printf "\n--== Test Suite ==--\n"
	{{_cargo}} test {{_build_flags}}

	# TODO: https://users.rust-lang.org/t/howto-sanitize-your-rust-code/9378

# Local Variables:
# mode: makefile
# End:

# vim: set ft=make textwidth=100 colorcolumn=101 noexpandtab sw=8 sts=8 ts=8 :
