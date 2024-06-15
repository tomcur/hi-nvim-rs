#!/usr/bin/env python3

from dataclasses import dataclass
import html
import itertools
import numpy
import os
import pyte
import subprocess
import sys

# Emulated terminal dimensions
LINES = 21
COLUMNS = 60

# Glyph monospace width vs height
FONT_ASPECT_RATIO = 0.6
FONT_ASCENT = 0.750


@dataclass
class TextStyle:
    color: str
    bold: bool
    italics: bool
    underscore: bool
    strikethrough: bool


def style_from_pyte_char(char) -> TextStyle:
    return TextStyle(
        char.fg, char.bold, char.italics, char.underscore, char.strikethrough
    )


def show_rect(color: str, x0: int, y0: int, x1: int, y1: int):
    return f"""<rect x="{x0*FONT_ASPECT_RATIO}" y="{y0}" width="{(x1 - x0 + 1) * FONT_ASPECT_RATIO}" height="{y1 - y0 + 1}" style="fill: #{color}" />"""


def show_text(text: str, style: TextStyle, x: int, y: int):
    style_ = f"fill: #{style.color}"
    if style.bold:
        style_ += "; font-weight: 600"
    if style.italics:
        style_ += "; font-style: italic"
    if style.underscore or style.strikethrough:
        style_ += "; text-decoration:"
        if style.underscore:
            style_ += " underline"
        if style.strikethrough:
            style_ += " line-through"
    return f"""<text x="{x*FONT_ASPECT_RATIO}" y="{y+FONT_ASCENT}" style="{style_}">{text}</text>"""


if __name__ == "__main__":
    env = os.environ.copy()
    env.update(dict(TERM="linux", COLUMNS=f"{COLUMNS}", LINES=f"{LINES}"))

    out = ""
    proc = subprocess.run(
        sys.argv[1:],
        capture_output=True,
        env=env,
    )
    out = proc.stdout

    screen = pyte.Screen(COLUMNS, LINES)
    stream = pyte.ByteStream(screen)
    stream.feed(out)

    print(
        f"""<svg viewBox="0 0 {COLUMNS*FONT_ASPECT_RATIO} {LINES}" xmlns="http://www.w3.org/2000/svg">"""
    )
    print(
        """
      <style>
        .screen {
            font-family: "Source Code Pro";
            font-size: 1px;
        }
      </style>
      <g class="screen">
    """
    )

    # find rectangles to draw by greedily flooding lines then flooding
    # down columns
    drawn = numpy.full((LINES, COLUMNS), False)
    for y, x in itertools.product(range(LINES), range(COLUMNS)):
        if drawn[y][x]:
            continue

        char = screen.buffer[y][x]
        bg = char.bg

        end_x = x
        end_y = y

        for x2 in range(x + 1, COLUMNS):
            char = screen.buffer[y][x2]
            if char.bg == bg:
                end_x = x2
            else:
                break

        for y2 in range(y + 1, LINES):
            all = True
            for x2 in range(x, end_x + 1):
                char = screen.buffer[y2][x2]
                if char.bg != bg:
                    all = False
                    break
            if not all:
                break
            end_y = y2

        for y2, x2 in itertools.product(range(y, end_y + 1), range(x, end_x + 1)):
            drawn[y2, x2] = True

        print(show_rect(bg, x, y, end_x, end_y))

    for y in range(LINES):
        char = screen.buffer[y][0]
        style = style_from_pyte_char(char)
        start_x = 0
        text = ""

        for x in range(COLUMNS):
            char = screen.buffer[y][x]
            style_ = style_from_pyte_char(char)

            if style_ != style:
                if not text == "":
                    print(show_text(text, style, start_x, y))
                text = ""
                style = style_

            if text == "":
                start_x = x
                if char.data == " ":
                    continue

            if char.data == " ":
                text += "&#160;"
            else:
                text += html.escape(char.data)

        if not text == "":
            print(show_text(text, style, start_x, y))

    print("</g></svg>")
