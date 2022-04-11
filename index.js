// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import init, { create_tree_map as createTreeMapWasm } from "./pkg/treemap_rust.js";

const runWasm = async () => {
  // Instantiate our wasm module
  await init("./pkg/treemap_rust_bg.wasm");

  // Call the Add function export from wasm, save the result
  const items = [6.0, 6.0, 4.0, 3.0, 2.0, 2.0, 1.0];
  const total = items.reduce((partialSum, a) => partialSum + a, 0);
  const w = Math.sqrt(total);
  const h = w;
  const bounds = [0, 0, w, h];
  const addResult = createTreeMapWasm(items, bounds);

  // Set the data onto the body
  document.body.querySelector('#input').value = `${JSON.stringify(items, null, '\t')}`;

  // Set the result onto the body
  document.body.querySelector('#output').value = `${JSON.stringify(addResult, null, '\t')}`;
};
runWasm();