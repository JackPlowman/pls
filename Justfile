# ------------------------------------------------------------------------------
# Rust Commands
# ------------------------------------------------------------------------------

# Check for Rust clippy issues
rust-lint-check:
    cargo clippy

# Fix Rust clippy issues
rust-lint-fix:
    cargo clippy --fix

# Check for Rust formatting issues
rust-fmt-check:
    cargo fmt -- --check

# Fix Rust formatting issues
rust-fmt:
    cargo fmt

# ------------------------------------------------------------------------------
# Prettier - File Formatting
# ------------------------------------------------------------------------------

# Check for prettier issues
prettier-check:
    prettier . --check

# Fix prettier issues
prettier-format:
    prettier . --check --write

# ------------------------------------------------------------------------------
# Justfile
# ------------------------------------------------------------------------------

# Format the Just code
just-format:
    just --fmt --unstable

# Check for Just format issues
just-format-check:
    just --fmt --check --unstable

# ------------------------------------------------------------------------------
# Git Hooks
# ------------------------------------------------------------------------------

# Install pre commit hook to run on all commits
install-git-hooks:
    cp -f githooks/pre-commit .git/hooks/pre-commit
    cp -f githooks/post-commit .git/hooks/post-commit
    chmod ug+x .git/hooks/*
