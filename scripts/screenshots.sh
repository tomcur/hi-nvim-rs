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
        echo ":set nowrap"
        echo ":LspStart"
        sleep 0.3
        echo ":Trouble lsp_document_symbols"
        echo ":NvimTreeOpen"
        printf '\x17l\r'
        sleep 0.3
        echo ":set splitright"
        echo ":55vsplit ./Cargo.toml"
        printf '\x17h\r/Cli\rz\r/fn main\r'
        sleep 3.0
    ) | termsnap --term xterm-256color --columns 160 --lines 45 --render-before-clear -- \
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
import re

def set_dimensions(svg: str, x: int, y: int, width: float, height: float) -> str:
  return svg.replace("<svg", f"""<svg x="{x}" y="{y}" width="{width}" height="{height}" """)

template = Path("./media/combined.svg").read_text()

img1 = Path("./media/highlow.svg").read_text()
img2 = Path("./media/twocolor.svg").read_text()
img3 = Path("./media/verf.svg").read_text()

p = re.compile(r'.*viewBox="\d+ \d+ (\d+) (\d+)"')
aspect_ratio = img1.partition('\n')[0]
match = p.match(aspect_ratio)
width = float(match.groups()[0])
height = float(match.groups()[1])
ratio = height/width

PADDING = 5
GAP = 25

total_width = PADDING * 2 + GAP * 2 + 100
total_height = PADDING * 2 + GAP * 2 + 100 * ratio

img1 = set_dimensions(Path("./media/highlow.svg").read_text(), PADDING, PADDING, 100, 100 * ratio)
img2 = set_dimensions(Path("./media/twocolor.svg").read_text(), PADDING + GAP, PADDING + GAP, 100, 100 * ratio)
img3 = set_dimensions(Path("./media/verf.svg").read_text(), PADDING + GAP * 2, PADDING + GAP * 2, 100, 100 * ratio)

template = template.replace("{width}", str(total_width))
template = template.replace("{height}", str(total_height))
template = template.replace("{img1}", img1)
template = template.replace("{img2}", img2)
template = template.replace("{img3}", img3)

Path("./media/combined-embedded.svg").write_text(template)
EOF
