import init from "../../crates/paycrypt-wasm/pkg/paycrypt_wasm.js";
import App from "./App.svelte";
import { mount } from "svelte";

// Initialise WASM before mounting so every value shown comes from the real crate.
await init();

const app = mount(App, { target: document.getElementById("app")! });
export default app;
