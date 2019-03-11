#!/bin/sh

if [ $# -lt 1 ]; then
    echo "Usage: $0 <path to new project directory> [...]"
    exit 1
fi

SELF_PATH="$(readlink -f "$0")"

SRC_PATH="$(dirname "$SELF_PATH")"
SELF_NAME="$(basename "$SELF_PATH")"

AUTHORS="$(git config --get user.name)  <$(git config --get user.email)>"

for target in "$@"; do (
    git clone -q -- "$SRC_PATH" "$target"

    cd -- "$target"
    rm -- "$SELF_NAME"

    if [ -e .genignore ]; then
        sed 's@\(^/\|\.\./\?\)@@g' .genignore | xargs rm --
        rm .genignore
    fi

    sed -i "s@{{\s*authors\s*}}@${AUTHORS}@" Cargo.toml
    sed -i "s@{{\s*project-name\s*}}@$(basename -- "$target")@" Cargo.toml
    find . -iname '*.rs' -exec sed -i "s@{{\s*authors\s*}}@${AUTHORS}@" {} \;
    find . -iname '*.rs' -exec sed -i "s@{{\s*\"now\"\s*|\s*date:\s*\"%Y\"\s*}}@$(date +%Y )@" {} \;

    rm -rf .git
    git init -q &&
    git add . &&
    git commit -qm "Created new project from template" &&
    echo "Created new project at $target"
); done
