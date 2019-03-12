# Version 0.2
# Copyright 2017-2019, Stephan Sokolow

# --== Variables to be customized/overridden by the user ==--

channel = "stable"
target = "i686-unknown-linux-musl"
features = ""

build_flags = "--release"
strip_bin = "strip"
strip_flags = "--strip-unneeded"
upx_flags = "--ultra-brute"
callgrind_args = ""
callgrind_out_file = "callgrind.out.justfile"

bash_completion_dir = "~/.bash_completion.d"
fish_completion_dir = "~/.config/fish/completions"
zsh_completion_dir = "~/.zsh/functions"
# Examples for OpenPandora cross-compilation
# target = "arm-unknown-linux-gnueabi"
# strip_bin = `echo $HOME/opt/pandora-dev/arm-2011.09/bin/pandora-strip`

# --== Code Begins ==--

# Parse the value of the "name" key in the [package] section of Cargo.toml
# using only the commands any POSIX-compliant platform should have
# Source: http://stackoverflow.com/a/40778047/435253
export zz_pkgname=`sed -nr "/^\[package\]/ { :l /^name[ ]*=/ { s/.*=[ ]*//; p; q;}; n; b l;}" Cargo.toml | sed 's@^"\(.*\)"$@\1@'`
export zz_target_path="target/" + target  + "/release/" + zz_pkgname

# Shorthand for `just test`
DEFAULT: test

# Alias for `cargo bloat --release {{args}}` with the default toolchain
bloat +args="":
	cargo bloat --release {{args}}

# Call `cargo build`
build:
	@echo "\n--== Building with {{channel}} for {{target}} (features: {{features}}) ==--\n"
	cargo "+{{channel}}" build --target="{{target}}" --features="{{features}}" {{build_flags}}

# Call `build` and then strip and compress the resulting binary
build-release: build
	cp "{{zz_target_path}}" "{{zz_target_path}}.packed"
	@printf "\n--== Stripping, SStripping, and Compressing With UPX ==--\n"
	{{strip_bin}} {{strip_flags}} "{{zz_target_path}}.packed"
	@# Allow sstrip to fail because it can't be installed via "just install-deps"
	sstrip "{{zz_target_path}}.packed" || true
	@# Allow upx to fail in case the user wants to force no UPXing by leaving it uninstalled
	upx {{upx_flags}} "{{zz_target_path}}.packed" || true
	@printf "\n--== Final Result ==--\n"
	@ls -1sh "{{zz_target_path}}" "{{zz_target_path}}.packed"
	@printf "\n"

# Alias for `cargo check {{args}}`
check +args="":
	cargo "+{{channel}}" check --target="{{target}}" --features="{{features}}" {{build_flags}} {{args}}

# Alias for `cargo clean -v {{args}}` which also deletes dist/
clean +args="":
	cargo clean -v {{args}}
	rm -rf dist
# Build the shell completions and a help file, and put them in a "dist" folder
dist-supplemental:
	mkdir -p dist
	@# Generate bash completion in dist/
	cargo "+{{channel}}" run --target="{{target}}" --features="{{features}}" {{build_flags}} \
		-- --dump-completions bash > dist/{{ zz_pkgname }}.bash
	@# Generate zsh completion in dist/
	cargo "+{{channel}}" run --target="{{target}}" --features="{{features}}" {{build_flags}} \
		-- --dump-completions zsh > dist/{{ zz_pkgname }}.zsh
	@# Generate fish completion in dist/
	cargo "+{{channel}}" run --target="{{target}}" --features="{{features}}" {{build_flags}} \
		-- --dump-completions fish > dist/{{ zz_pkgname }}.fish

# alias for `cargo doc --document-private-items {{args}}` with the default toolchain
doc +args="":
	cargo doc --document-private-items --target="{{target}}" --features="{{features}}" {{build_flags}} {{args}}

# Alias for `cargo +nightly fmt -- {{args}}`
fmt +args="":
	cargo +nightly fmt -- {{args}}

# Alias for `cargo +nightly fmt -- --check {{args}} which un-bloats TODO/FIXME warnings
fmt-check +args="":
	cargo +nightly fmt -- --check --color always {{args}} 2>&1 | egrep -v '[0-9]*[ ]*\|'

# Use `apt-get` to install dependencies `cargo` can't (except `kcov` and `sstrip`)
install-apt-deps:
	sudo apt-get install binutils kcachegrind upx valgrind

# `install-rustup-deps` and then `cargo install` tools
install-cargo-deps: install-rustup-deps
	@# Prevent "already installed" from causing a failure
	cargo install cargo-deadlinks || true
	cargo install cargo-bloat || true
	cargo install cargo-outdated || true

# Install (don't update) nightly `channel` toolchains, plus `target`, clippy, and rustfmt
install-rustup-deps:
	@# Prevent this from gleefully doing an unwanted "rustup update"
	rustup toolchain list | grep -q '{{channel}}' || rustup toolchain install '{{channel}}'
	rustup toolchain list | grep -q nightly || rustup toolchain install nightly
	rustup target list | grep -q '{{target}} (' || rustup target add '{{target}}'
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
	cargo build --target="{{target}}" --features="{{features}}" {{build_flags}}
	rm -rf '{{ callgrind_out_file }}'
	valgrind --tool=callgrind --callgrind-out-file='{{ callgrind_out_file }}' {{ callgrind_args }} 'target/debug/{{ zz_pkgname }}' '{{ args }}' || true
	test -e '{{ callgrind_out_file }}'
	kcachegrind '{{ callgrind_out_file }}'

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

	# Ensure that kcov can see totally unused functions
	# Sources:
	# - http://stackoverflow.com/a/38371687/435253
	# - https://gist.github.com/dikaiosune/07177baf5cea76c27783efa55e99da89
	export RUSTFLAGS='-C link-dead-code'
	cargo clean -v

	shift
	cargo test --no-run --target="{{target}}" --features="{{features}}" {{build_flags}} || exit $?
	rm -rf target/cov

	for file in target/debug/$zz_pkgname-*; do
		if [ -x "$file" ]; then
			mkdir -p "target/cov/$(basename $file)"
			kcov --exclude-pattern=/.cargo,/usr/lib --verify "target/cov/$(basename $file)" "$file" "$@"
		fi
	done

# Alias for `cargo run -- {{args}}`
run +args="":
	cargo "+{{channel}}" run --target="{{target}}" --features="{{features}}" {{build_flags}} -- {{args}}

# Run all installed static analysis, plus `cargo test`.
test:
	@echo "--== Outdated Packages ==--"
	cargo outdated
	@printf "\n--== Clippy Lints ==--\n"
	cargo clippy --target="{{target}}" --features="{{features}}" {{build_flags}}
	@printf "\n--== Dead Internal Documentation Links ==--\n"
	cargo doc --document-private-items  --target="{{target}}" --features="{{features}}" {{build_flags}} && cargo deadlinks
	@printf "\n--== Test Suite ==--\n"
	cargo test --target="{{target}}" --features="{{features}}" {{build_flags}}

	# TODO: https://users.rust-lang.org/t/howto-sanitize-your-rust-code/9378

# Local Variables:
# mode: makefile
# End:

# vim: set ft=make textwidth=100 colorcolumn=101 noexpandtab sw=8 sts=8 ts=8 :
