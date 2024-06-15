#!/usr/bin/env bash

set -e

DIR=$(mktemp -d)

for light_dark in "light" "dark"; do
    for colorscheme in "highlow" "twocolor" "grayscale"; do
        cargo run -- ../colorschemes/$colorscheme.toml >"$DIR/$colorscheme.vim"

        ./capture.py nvim --clean \
            -c "source ./nvim-config.vim" \
            -c "set bg=$light_dark" \
            -c "source $DIR/$colorscheme.vim" \
            `# defer to ensure neovim tui has settled` \
            -c "lua vim.defer_fn(function() vim.cmd('q') end, 0)" \
            ../hi-nvim-rs-web/code-examples/rust.rs \
            >../media/$colorscheme-$light_dark.svg
    done
done
