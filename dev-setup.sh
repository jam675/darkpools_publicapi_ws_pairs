#!/usr/bin/env bash

# Section: Helper functions

function command_exists_or_die() {
    if [ -z $(command -v $1) ]; then
        echo -e "\e[1merror:\e[0m cannot find $1." 1>&2
        exit 1
    fi
}

function rust_add() {
    rustup component add "$1"
    if [ "$?" -gt 0 ]; then
        echo -e "\e[1merror:\e[0m cannot proceed." 1>&2
        exit 1
    fi
}

function cargo_utilities() {
    echo -e "\e[1minfo:\e[0m installing $1..."
    cargo install "$1" 1>/dev/null 2>&1
}

# Section: Install pre-commit hook to automatically run 'rustftm' then re-add the changes

if test -f ".git/hooks/pre-commit"; then
    echo -e "\e[1mwarning:\e[0m .git/hooks/pre-commit exists, skipping..."
else
    echo -e "\e[1minfo:\e[0m installing pre-commit hook.."
    touch .git/hooks/pre-commit
    cat > .git/hooks/pre-commit <<\EOF
#!/bin/bash
staged_files=$(git diff --name-only --staged --diff-filter=dr)
if [[ -z ${staged_files} ]]; then
    cat <<\EOF1
No staged file(s), please do `git add ...`
EOF1
    exit 1
fi
flatten_staged_files=$(git diff --name-only --staged --diff-filter=dr | grep -i ".rs" | tr "\n" " ")
if [[ -z ${flatten_staged_files} ]]; then
    exit 0
fi
cargo fmt -- ${flatten_staged_files}
if [[ $? -ne 0 ]]; then
    cat <<\EOF1
There is a problem with `rustfmt`, ensure it is properly installed.
EOF1
    exit 1
fi
git add ${staged_files}
exit 0
EOF
    chmod +x .git/hooks/pre-commit
    echo -e "\e[1minfo:\e[0m pre-commit hook installed."
fi

# Section: Rust components and mandatory cargo crates (system-wide!)

command_exists_or_die "rustup"
rustup default stable
rustup update
rust_add "rustc"
rust_add "rust-src"
rust_add "rust-docs"
rust_add "rust-analysis"
rust_add "cargo"
rust_add "clippy"
rust_add "rustfmt"
cargo_utilities "cargo-audit"
echo -e "\e[1mdone.\e[0m"
