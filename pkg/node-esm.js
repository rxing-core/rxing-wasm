// dist/node-esm-auto.js
import * as shim from "./rxing_wasm_bg.js";
export * from "./rxing_wasm_bg.js";

import { readFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import { dirname, resolve } from "node:path";

const __filename = fileURLToPath(import.meta.url);
const __dirname  = dirname(__filename);

await (async () => {
  const wasmPath = resolve(__dirname, "./rxing_wasm_bg.wasm");
  const bytes = await readFile(wasmPath);

  // compile to discover the expected import module name (e.g. "wbg" or "./rxing_wasm_bg.js")
  const mod = new WebAssembly.Module(bytes);
  const moduleKey = WebAssembly.Module.imports(mod)[0]?.module || "wbg";

  const instance = new WebAssembly.Instance(mod, { [moduleKey]: shim });

  const setWasm =
    shim.__wbg_set_wasm ||
    shim.__wbindgen_set_wasm ||
    shim.__wbg_setWasm ||
    shim.set_wasm;

  if (typeof setWasm !== "function") {
    throw new Error("rxing-wasm: wasm-bindgen set_wasm helper not found");
  }
  setWasm(instance.exports);
})();
