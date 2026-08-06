import init from "../../crates/paycrypt-wasm/pkg/paycrypt_wasm.js";
import App from "./App.svelte";

// Initialise WASM before mounting so every value shown comes from the real crate.
await init();

const app = new App({ target: document.getElementById("app")! });
export default app;
