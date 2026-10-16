"""Draw the exact initial cells from the leaf engine example as a README badge.

Usage: cargo run --quiet -p lifelike --example leaf | python3 scripts/render_leaf_badge.py
Requires Pillow. No simulation is performed in this script.
"""
import json
import sys
from pathlib import Path

from PIL import Image, ImageDraw


def main():
    data = json.load(sys.stdin)
    width = data["width"]
    frame = data["frames"][0]
    points = [(index // width, index % width)
              for index in range(width * data["height"])
              if frame[index // 8] & (1 << (7 - index % 8))]
    top = min(row for row, col in points)
    left = min(col for row, col in points)
    rows = max(row for row, col in points) - top + 1
    cols = max(col for row, col in points) - left + 1
    pitch, cell = 72, 58
    x0 = (1024 - (cols - 1) * pitch - cell) // 2
    y0 = (1024 - (rows - 1) * pitch - cell) // 2
    image = Image.new("RGBA", (1024, 1024), (0, 0, 0, 0))
    draw = ImageDraw.Draw(image)
    draw.ellipse((32, 32, 991, 991), fill="#020503", outline="#8ce8ae", width=5)
    rects = []
    for row, col in points:
        x, y = x0 + (col - left) * pitch, y0 + (row - top) * pitch
        draw.rectangle((x, y, x + cell - 1, y + cell - 1), fill="#8ce8ae")
        rects.append(f'<rect x="{x}" y="{y}" width="{cell}" height="{cell}"/>')
    output = Path("docs/assets")
    image.save(output / "lifelike-leaf.png", optimize=True)
    (output / "lifelike-leaf.svg").write_text(
        '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024" role="img" aria-labelledby="title">\n'
        '<title id="title">Lifelike: a twenty-cell leaf-shaped HighLife seed</title>\n'
        '<circle cx="512" cy="512" r="477" fill="#020503" stroke="#8ce8ae" stroke-width="5"/>\n'
        '<g fill="#8ce8ae">\n' + '\n'.join(rects) + '\n</g>\n</svg>\n'
    )
    print(f"Rendered {len(points)} exact engine cells as PNG and SVG", file=sys.stderr)


if __name__ == "__main__":
    main()
