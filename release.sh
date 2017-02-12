#!/bin/sh
# Version 0.1

# Definitions for setting target platform
# 32-bit x86 with static libc
TARGET=i686-unknown-linux-musl
STRIP=strip
# OpenPandora
#TARGET=arm-unknown-linux-gnueabi
#STRIP=~/opt/pandora-dev/arm-2011.09/bin/pandora-strip

CHANNEL=stable
FEATURES=""
STRIP_FLAGS="--strip-unneeded"
export UPX="--ultra-brute"

PKGID="$(cargo pkgid)"
[ -z "$PKGID" ] && exit 1
PKGNAME="${PKGID#*\#}"
PKGNAME="${PKGNAME%:*}"
[ -z "$PKGNAME" ] && exit 1
TARGET_PATH="target/$TARGET/release/$PKGNAME"

# If --nightly, use opt-level=z and alloc_system to cut 115K from the output
if [ "$1" = "--nightly" ]; then
    CHANNEL=nightly
    FEATURES="$FEATURES --features=nightly"

    # TODO: Find a less hacky way to do this
    cleanup() {
        sed -i 's/opt-level = "z"/opt-level = 3/' Cargo.toml
    }
    trap cleanup EXIT
    sed -i 's/opt-level = 3/opt-level = "z"/' Cargo.toml
fi

# Always delete and rebuild (since stripping a UPXd executable is fatal)
rm "$TARGET_PATH"
# shellcheck disable=SC2086
rustup run "$CHANNEL" cargo build --release --target="$TARGET" $FEATURES

# Crunch down the resulting output using strip, sstrip, and upx
"$STRIP" $STRIP_FLAGS "$TARGET_PATH"
sstrip "$TARGET_PATH"  # from ELFkickers
upx "$TARGET_PATH"

# Display the result
ls -lh "$TARGET_PATH"
