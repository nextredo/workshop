#! /usr/bin/python3

import re
import sys

RESET = '\033[0m'
def get_color_escape(r: int, g: int, b: int, background=False):
    return '\033[{};2;{};{};{}m'.format(48 if background else 38, r, g, b)

def hex_to_rgb(input: str):
    # End stop not inclusive
    # r = int(hex[0:2], 16)
    # g = int(hex[2:4], 16)
    # b = int(hex[4:6], 16)

    matches = re.match(r"[0-9a-fA-F]{6}", input)

    r, g, b = 0x41, 0x41, 0x41
    if matches:
        matched_str = matches.group()
        r = int(matched_str[0:2], 16)
        g = int(matched_str[2:4], 16)
        b = int(matched_str[4:6], 16)

    return (r, g, b)

# ------------------------------------------------------------------------------
# Script start -----------------------------------------------------------------
if __name__ == "__main__":
    for line in sys.stdin:
        print(get_color_escape(*hex_to_rgb(line)) + "Colour: " + line + RESET)


# ------------------------------------------------------------------------------
# Usage ------------------------------------------------------------------------
# something along the lines of:
# echo 5D5D5D | ./hex_to_colour.py
