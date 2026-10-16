import init, { Universe, LifeRule } from "./pkg/lifelike_wasm.js";

// Each viewer owns its board and controls; no global styles or dependencies.
export async function mountLife(container, {
  width = 64,
  height = 40,
  cellSize = 8,
  interval = 100,
  rule = LifeRule.Conway,
  cells = [],
} = {}) {
  await init();
  const universe = new Universe(width, height, rule);
  for (const [row, col] of cells) universe.setCell(row, col, true);

  const root = document.createElement("div");
  root.className = "life-viewer";
  const status = document.createElement("div");
  status.className = "life-status";
  const generationLabel = document.createElement("span");
  const populationLabel = document.createElement("span");
  status.append(generationLabel, populationLabel);
  let generation = 0;
  const canvas = document.createElement("canvas");
  canvas.width = width * cellSize;
  canvas.height = height * cellSize;
  canvas.style.cssText = "display:block;max-width:100%;height:auto;cursor:crosshair;";
  canvas.setAttribute("aria-label", "Cellular automaton. Click to edit cells.");
  const context = canvas.getContext("2d");
  const controls = document.createElement("div");
  controls.className = "life-controls";
  const play = document.createElement("button");
  play.className = "life-play";
  const step = document.createElement("button");
  const clear = document.createElement("button");
  clear.className = "life-clear";
  const reset = document.createElement("button");
  reset.textContent = "Reset";
  for (const button of [play, step, reset, clear]) button.type = "button";
  play.textContent = "Start";
  step.textContent = "Step";
  clear.textContent = "Clear";
  controls.append(play, step, reset, clear);
  root.append(status, canvas, controls);
  container.append(root);

  function draw() {
    const bytes = universe.cells();
    let population = 0;
    context.fillStyle = "#020503";
    context.fillRect(0, 0, canvas.width, canvas.height);
    context.fillStyle = "#8ce8ae";
    for (let row = 0; row < height; row++) {
      for (let col = 0; col < width; col++) {
        const index = row * width + col;
        if (bytes[index >> 3] & (1 << (7 - index % 8))) {
          population++;
          context.fillRect(col * cellSize, row * cellSize, Math.max(1, cellSize - 1), Math.max(1, cellSize - 1));
        }
      }
    }
    generationLabel.textContent = `GENERATION ${generation.toLocaleString("en")}`;
    populationLabel.textContent = `${population.toLocaleString("en")} LIVING CELLS`;
  }

  let timer = null;
  function stop() {
    clearInterval(timer);
    timer = null;
    play.textContent = "Start";
  }
  play.onclick = () => {
    if (timer !== null) {
      stop();
    } else {
      timer = setInterval(() => { universe.tick(); generation++; draw(); }, interval);
      play.textContent = "Pause";
    }
  };
  step.onclick = () => { stop(); universe.tick(); generation++; draw(); };
  clear.onclick = () => { stop(); universe.clear(); generation = 0; draw(); };
  reset.onclick = () => {
    stop();
    universe.clear();
    for (const [row, col] of cells) universe.setCell(row, col, true);
    generation = 0;
    draw();
  };
  canvas.onclick = (event) => {
    const bounds = canvas.getBoundingClientRect();
    const col = Math.floor((event.clientX - bounds.left) / bounds.width * width);
    const row = Math.floor((event.clientY - bounds.top) / bounds.height * height);
    universe.setCell(row, col, !universe.getCell(row, col));
    draw();
  };
  draw();

  return {
    destroy() {
      stop();
      canvas.onclick = play.onclick = step.onclick = reset.onclick = clear.onclick = null;
      root.remove();
      universe.free();
    },
  };
}

export { LifeRule };
