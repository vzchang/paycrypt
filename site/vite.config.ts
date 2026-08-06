import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

// The WASM package comes from `wasm-pack build` into ../crates/paycrypt-wasm/pkg.
export default defineConfig({
  plugins: [svelte(), wasm(), topLevelAwait()],
  base: "./",
  build: {
    target: "esnext",
  },
});
