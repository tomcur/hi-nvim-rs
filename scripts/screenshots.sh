#!/usr/bin/env bash

set -e

# This script expects to be run from the repository's root directory.

DIR=$(mktemp -d)

NVIM="nvim"
if [ -n "$SCREENSHOT_NVIM" ]; then
    NVIM="$SCREENSHOT_NVIM"
fi

for light_dark in "light" "dark"; do
    for colorscheme in "highlow" "verf" "twocolor" "grayscale"; do
        cargo run -- ./colorschemes/$colorscheme.toml >"$DIR/$colorscheme.vim"

        (
            # exit on a timer to perform some other commands in the meantime
            echo ":lua vim.defer_fn(function() vim.cmd('q!') end, 50)"
            echo -ne ":12\rnVjj$"
            sleep 0.1
        ) | termsnap --term xterm-256color --columns 58 --lines 20 --render-before-clear -- \
            "$NVIM" --clean \
            -c "source ./scripts/nvim-config.vim" \
            -c "set bg=$light_dark" \
            -c "source $DIR/$colorscheme.vim" \
            ./hi-nvim-rs-web/code-examples/rust.rs \
            >./media/$colorscheme-$light_dark.svg
    done
done
