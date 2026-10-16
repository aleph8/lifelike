"""Render packed board frames from stdin; simulation comes from the Rust engine.

Requires Pillow. Usage:
    cargo run --quiet -p lifelike --example replicator | python scripts/render_animation.py
"""
import json
import sys
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont


def main():
    data = json.load(sys.stdin)
    width, height = data["width"], data["height"]
    scale, header = 10, 64
    palette = [2, 5, 3, 140, 232, 174, 125, 153, 135] + [0] * (768 - 9)
    font = ImageFont.load_default(size=16)
    frames = []
    for generation, cells in enumerate(data["frames"]):
        frame = Image.new("P", (width * scale, height * scale + header), 0)
        frame.putpalette(palette)
        draw = ImageDraw.Draw(frame)
        population = 0
        for row in range(height):
            for col in range(width):
                index = row * width + col
                if cells[index // 8] & (1 << (7 - index % 8)):
                    population += 1
                    x, y = col * scale, row * scale + header
                    draw.rectangle((x, y, x + scale - 2, y + scale - 2), fill=1)
        draw.text((18, 12), data.get("title", "HIGHLIFE / REPLICATOR"), font=font, fill=1)
        draw.text((18, 36), f"{data.get('rule', 'B36/S23')}    Generation {generation:02d}    Living cells {population}", font=font, fill=2)
        frames.append(frame)
    output = Path(sys.argv[1] if len(sys.argv) > 1 else "docs/assets/replicator.gif")
    output.parent.mkdir(parents=True, exist_ok=True)
    durations = [data.get("duration", 120)] * len(frames)
    durations[0] = durations[-1] = data.get("pause", 1500)
    frames[0].save(output, save_all=True, append_images=frames[1:], duration=durations, loop=0, optimize=False, disposal=2)
    print(f"Saved {len(frames)} frames to {output}", file=sys.stderr)


if __name__ == "__main__":
    main()
