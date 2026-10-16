import { mountLife, LifeRule } from "./viewer.js";

const patterns = [
  {
    name: "Glider", rule: LifeRule.Conway, rulestring: "Conway · B3/S23",
    description: "A tiny spaceship that travels diagonally across the board.",
    rows: [".O.", "..O", "OOO"],
  },
  {
    name: "Replicator", rule: LifeRule.HighLife, rulestring: "HighLife · B36/S23",
    description: "A HighLife pattern that makes two copies of itself after 12 generations.",
    // https://conwaylife.com/wiki/OCA:HighLife
    rows: ["..OOO", ".O..O", "O...O", "O..O.", "OOO.."],
  },
  {
    name: "Blinker", rule: LifeRule.Conway, rulestring: "Conway · B3/S23",
    description: "Three cells, two states. The simplest oscillator.",
    rows: ["OOO"],
  },
  {
    name: "Toad", rule: LifeRule.Conway, rulestring: "Conway · B3/S23",
    description: "Two short rows that open and close in a two-generation cycle.",
    rows: [".OOO", "OOO."],
  },
  {
    name: "R-pentomino", rule: LifeRule.Conway, rulestring: "Conway · B3/S23",
    description: "Five cells that unfold into a long, complex evolution. Try letting it run.",
    rows: [".OO", "OO.", ".O."],
  },
  {
    name: "Lifelike Leaf", rule: LifeRule.HighLife, rulestring: "HighLife · B36/S23",
    description: "The 20-cell leaf behind the Lifelike emblem. Watch it unfold in HighLife.",
    rows: ["......OO", "....OO.O", "...O..OO", "..O.O.O.", "..OO.O..", ".O.OO...", ".OO.....", "O......."],
  },
];

const select = document.querySelector("#pattern");
const container = document.querySelector("#life");
const error = document.querySelector("#error");
let viewer;
for (const [index, pattern] of patterns.entries()) {
  select.add(new Option(pattern.name, index));
}

async function loadPattern() {
  select.disabled = true;
  error.textContent = "";
  try {
    const pattern = patterns[Number(select.value)];
    const width = 80;
    const height = 48;
    const top = Math.floor((height - pattern.rows.length) / 2);
    const left = Math.floor((width - pattern.rows[0].length) / 2);
    const cells = pattern.rows.flatMap((row, r) => [...row].flatMap((cell, c) =>
      cell === "O" ? [[top + r, left + c]] : []));
    viewer?.destroy();
    viewer = await mountLife(container, { width, height, cellSize: 10, rule: pattern.rule, cells });
    document.querySelector("#rule").textContent = pattern.rulestring;
    document.querySelector("#description").textContent = pattern.description;
  } catch (cause) {
    error.textContent = `Could not load the viewer: ${cause.message}`;
  } finally {
    select.disabled = false;
  }
}

select.addEventListener("change", loadPattern);
await loadPattern();
