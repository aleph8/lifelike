A Rust engine for Conway's Game of Life, HighLife, and Seeds, compiled to WebAssembly.

### Downloads

- **lifelike-wasm-v*.zip**: the complete browser ES module, WASM binary, TypeScript declarations, package metadata, and MIT license. Extract all files into the same directory.
- Individual `.js`, `.wasm`, and `.d.ts` files are also attached.
- **SHA256SUMS**: checksums for all downloads.

### Browser usage

Serve the extracted files over HTTP and import the module:

```html
<script type="module">
  import init, { Universe, LifeRule } from "./lifelike_wasm.js";

  await init();
  const universe = new Universe(64, 48, LifeRule.Conway);
  universe.setCell(2, 3, true);
  universe.tick();
  console.log(universe.cells());
  universe.free();
</script>
```

The JavaScript loader expects `lifelike_wasm_bg.wasm` beside it. See the repository README and `examples/` for the interactive viewer and embedding instructions.
