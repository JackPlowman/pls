#!/bin/bash

# Build the binary
cargo build --release

# Copy binary to a location in PATH
sudo cp target/release/pls /usr/local/bin/pls-bin

# Create shell function
SHELL_CONFIG="$HOME/.$(basename $SHELL)rc"
echo '
# pls - directory navigator
pls() {
    local result_file=$(mktemp)
    pls-bin > "$result_file"
    local last_line=$(tail -n 1 "$result_file")
    rm "$result_file"
    if [[ $last_line == cd* ]]; then
        eval "$last_line"
    fi
}' >> "$SHELL_CONFIG"

echo "Installation complete! Please restart your shell or run: source $SHELL_CONFIG"
