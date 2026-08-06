# paycrypt explainer site

Interactive explainer that runs the **real compiled `paycrypt` library** via
WebAssembly and animates the DUKPT key ladder and ISO-0 PIN block
construction, exposing the intermediate derivation state every existing public
calculator hides. Live at **https://vzchang.github.io/paycrypt/**.

## Status

- The Rust WASM crate (`crates/paycrypt-wasm`) builds and is verified: a
  `wasm-pack build` produces a ~40 KB `.wasm` plus TypeScript declarations for
  `tdes_ladder_steps`, `iso0_steps`, and `version`. Its DTO logic is covered by
  native tests pinned to the library's published KATs.
- The frontend (this `site/` directory) **builds and is verified**: `vite build`
  produces a complete static bundle in `dist/` (HTML + JS + CSS + the real
  `.wasm`). It uses the Vite 4 / Svelte 4 toolchain, which runs on Node 16+
  (chosen deliberately so it builds on older-glibc hosts; Vite 5 requires
  Node 18+ and its `crypto.getRandomValues` global).

## Local dev

Prerequisites: Rust with the `wasm32-unknown-unknown` target, `wasm-pack`, and
Node 16+.

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack

cd site
npm install
npm run dev      # runs wasm-pack build, then vite dev server
```

`npm run build` produces a static bundle in `site/dist/` (deployable to any
static host; serve `.wasm` as `application/wasm`). The `wasm` npm script
regenerates `../crates/paycrypt-wasm/pkg` from the Rust source, so the site
always runs the current library.

## Manual checks

- The page loads and shows the `version()` string (module initialised).
- Dragging the transaction-counter slider changes the final derived key.
- The step controls walk the ladder and highlight which bytes changed.
- `prefers-reduced-motion` degrades animation to instant state changes.
