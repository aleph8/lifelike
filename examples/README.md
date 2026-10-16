# Minimal viewer

HTML and JavaScript with no dependencies or bundler. Click to edit cells,
start or pause the simulation, advance one generation, and clear the board.
The example starts with a glider. Select Glider, Replicator, Blinker, Toad,
R-pentomino, or Lifelike Leaf from the pattern menu. Each selection loads the matching rule and
centers the pattern. Lifelike Leaf uses the same 20-cell HighLife seed as the
README emblem. Reset restores the initial pattern; Clear empties the board.
Generation and population counters update as the simulation runs.

The HighLife replicator is documented on [LifeWiki](https://conwaylife.com/wiki/OCA:HighLife).

## Build

You need Rust and `wasm-pack`.
From the repository root:

```bash
rustup target add wasm32-unknown-unknown
wasm-pack build wasm --target web --out-dir ../examples/pkg
```

`--out-dir` is relative to the `wasm` crate: the JavaScript package, TypeScript
types, and WASM binary are generated in `examples/pkg/`, which Git ignores.
Run the second command again whenever you change the Rust code.

## Run the example

From the repository root, using Python 3:

```bash
python3 -m http.server 8000 --directory examples
```

Open http://localhost:8000 Use HTTP; opening `index.html` via `file://` does
not load the modules and WASM correctly.

## Embed in another website

Copy `viewer.js`, `viewer.css`, and the generated `pkg/` directory together into your website:

```text
life/
  viewer.js
  viewer.css
  pkg/
    ...generated files...
```

Add a container and a module:

```html
<link rel="stylesheet" href="/life/viewer.css">
<div id="life"></div>
<script type="module">
  import { mountLife, LifeRule } from "/life/viewer.js";

  const viewer = await mountLife(document.querySelector("#life"), {
    width: 64,
    height: 40,
    cellSize: 8,
    interval: 100,
    rule: LifeRule.Conway,
    cells: [[2, 3], [3, 4], [4, 2], [4, 3], [4, 4]],
  });

  // When removing the component: viewer.destroy();
</script>
```

All options are optional. `interval` is in milliseconds; `cells` contains
zero-based `[row, column]` pairs.
Available rules are `Conway`, `HighLife`, and `Seeds`.
Each viewer has its own board and controls. The optional `viewer.css` stylesheet
is scoped to `.life-viewer`; the centered black page layout belongs to the demo.
The canvas scales to fit the container width while preserving its aspect ratio.
