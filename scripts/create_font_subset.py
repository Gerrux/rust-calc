# -*- coding: utf-8 -*-
"""
Create a font subset with only the characters needed for rust-calc.
Requires: pip install fonttools brotli
"""

import sys
import os

# Force UTF-8 encoding
if sys.platform == 'win32':
    sys.stdout.reconfigure(encoding='utf-8')

from fontTools.subset import main as subset_main
import tempfile

# All characters needed for the calculator
CHARS = (
    # Digits
    "0123456789"
    # Basic letters (for functions, labels)
    "abcdefghijklmnopqrstuvwxyz"
    "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    # Punctuation
    ".,;:!?'\"-+=*/\\()[]{}|_<>@#$%&^`~"
    # Math operators (Unicode)
    "\u00D7"  # × multiplication
    "\u00F7"  # ÷ division
    "\u2212"  # − minus
    "\u00B1"  # ± plus-minus
    "\u221A"  # √ square root
    "\u00B2"  # ² superscript 2
    "\u02B8"  # ʸ superscript y
    "\u03C0"  # π pi
    "\u2E23"  # ⸣ corner bracket (for paren indicator)
    # Whitespace
    " "
)

def main():
    script_dir = os.path.dirname(os.path.abspath(__file__))
    project_root = os.path.dirname(script_dir)
    output_font = os.path.join(project_root, "assets", "font.ttf")

    # Source font - use Consola (Windows monospace) or fallback
    source_fonts = [
        "C:/Windows/Fonts/consola.ttf",      # Consolas
        "C:/Windows/Fonts/cour.ttf",          # Courier New
        "C:/Windows/Fonts/arial.ttf",         # Arial (fallback)
    ]

    source_font = None
    for font in source_fonts:
        if os.path.exists(font):
            source_font = font
            print(f"Using source font: {font}")
            break

    if not source_font:
        print("ERROR: No suitable source font found!")
        sys.exit(1)

    # Create unicodes string for fonttools
    unicodes = ",".join(f"U+{ord(c):04X}" for c in set(CHARS))

    # Create output directory if needed
    os.makedirs(os.path.dirname(output_font), exist_ok=True)

    # Run fonttools subset
    args = [
        source_font,
        f"--unicodes={unicodes}",
        f"--output-file={output_font}",
        "--no-hinting",           # Remove hinting (saves space)
        "--desubroutinize",       # Simplify font
    ]

    print(f"Creating font subset...")
    print(f"Characters: {len(set(CHARS))}")
    print(f"Output: {output_font}")

    # fonttools.subset.main expects sys.argv format
    sys.argv = ["fonttools-subset"] + args
    subset_main()

    # Report size
    size = os.path.getsize(output_font)
    print(f"Done! Font size: {size / 1024:.1f} KB")

if __name__ == "__main__":
    main()
