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

# prepare combined SVG from template
python3 <<EOF
from pathlib import Path

def set_dimensions(svg: str, x: int, y: int, width: int, height: int) -> str:
  return svg.replace("<svg", f"""<svg x="{x}" y="{y}" width="{width}" height="{height}" """)

template = Path("./media/combined.svg").read_text()

img1 = set_dimensions(Path("./media/highlow.svg").read_text(), 5, 5, 120, 70)
img2 = set_dimensions(Path("./media/twocolor.svg").read_text(), 35, 35, 120, 70)
img3 = set_dimensions(Path("./media/verf.svg").read_text(), 65, 65, 120, 70)

template = template.replace("{img1}", img1)
template = template.replace("{img2}", img2)
template = template.replace("{img3}", img3)

Path("./media/combined-embedded.svg").write_text(template)
EOF
