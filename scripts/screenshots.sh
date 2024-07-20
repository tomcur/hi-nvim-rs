#!/usr/bin/env bash

set -e

# This script expects to be run from the repository's root directory.

DIR=$(mktemp -d)

NVIM="nvim"
if [ -n "$SCREENSHOT_NVIM" ]; then
    NVIM="$SCREENSHOT_NVIM"
fi

cargo build --release
BIN=./target/release/hi-nvim-rs

for COLORSCHEME_AND_KIND in "highlow-light" "verf-dark" "twocolor-dark" "grayscale-dark"; do
    colorscheme=${COLORSCHEME_AND_KIND%-*}
    kind=${COLORSCHEME_AND_KIND#*-}

    # Small screenshots
    for light_dark in "light" "dark"; do
        $BIN -- ./colorschemes/"$colorscheme".toml >"$DIR/$colorscheme.vim"

        (
            # exit on a timer to perform some other commands in the meantime
            echo ":lua vim.defer_fn(function() vim.cmd('q!') end, 50)"
            echo -ne ":12\rnVjj$"
            sleep 0.5
        ) | termsnap --term xterm-256color --columns 58 --lines 20 --render-before-clear -- \
            "$NVIM" --clean \
            -c "source ./scripts/nvim-config.vim" \
            -c "set bg=$light_dark" \
            -c "source $DIR/$colorscheme.vim" \
            ./hi-nvim-rs-web/code-examples/rust.rs \
            >./media/"$colorscheme"-$light_dark.svg &
    done

    # Large screenshots
    (
        # exit on a timer to perform some other commands in the meantime
        echo ":lua vim.defer_fn(function() vim.cmd('qa!') end, 3000)"
        echo ":LspStart"
        sleep 0.3
        echo ":Trouble lsp_document_symbols"
        echo ":NvimTreeOpen"
        printf '\x17l\r'
        sleep 0.3
        echo ":set splitright"
        echo ":68vsplit ./Cargo.toml"
        printf '\x17h\r/fn main\r'
        sleep 3.0
    ) | termsnap --term xterm-256color --columns 200 --lines 70 --render-before-clear -- \
        "$NVIM" --clean \
        -c "source ./scripts/nvim-config.vim" \
        -c "set bg=$kind" \
        -c "source $DIR/$colorscheme.vim" \
        ./src/main.rs \
        >./media/"$colorscheme".svg &
done

wait
