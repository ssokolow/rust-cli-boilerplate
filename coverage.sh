#!/bin/bash
# Sources:
# - http://sunjay.ca/2016/07/25/rust-code-coverage
# - https://users.rust-lang.org/t/tutorial-how-to-collect-test-coverages-for-rust-project/650
#
# TODO:
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
ORIGIN="${ORIGIN:7}"
PKGNAMEVER="${PKGID#*#}"
PKGNAME="${PKGNAMEVER%:*}"

# XXX: Why do some `cargo pkgid` runs not contain the `name:` part?
if [ "$PKGNAME" = "$PKGNAMEVER" ]; then
    PKGNAME="${ORIGIN##*/}"
fi

# Ensure that kcov can see totally unused functions
# Sources:
# - http://stackoverflow.com/a/38371687/435253
# - https://gist.github.com/dikaiosune/07177baf5cea76c27783efa55e99da89
export RUSTFLAGS='-C link-dead-code'
cargo clean -v

shift
cargo test --no-run || exit $?
EXE=($ORIGIN/target/debug/$PKGNAME-*)
if [ ${#EXE[@]} -ne 1 ]; then
    echo 'Non-unique test file, retrying...' >2
    rm -f "${EXE[@]}"
    cargo test --no-run || exit $?
fi
rm -rf "$ORIGIN/target/cov"
kcov --exclude-pattern=/.cargo,/usr/lib --verify "$ORIGIN/target/cov" "$ORIGIN/target/debug/$PKGNAME-"* "$@"
