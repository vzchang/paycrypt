<script lang="ts">
  import "./app.css";
  import { version } from "../../crates/paycrypt-wasm/pkg/paycrypt_wasm.js";
  import DukptLadder from "./lib/DukptLadder.svelte";
  import PinBlockStepper from "./lib/PinBlockStepper.svelte";
  import FlowStrip from "./lib/FlowStrip.svelte";
  import ThemePicker from "./lib/ThemePicker.svelte";
  import MatrixRain from "./lib/MatrixRain.svelte";

  let theme = "";
</script>

<MatrixRain active={theme === "matrix"} />
<ThemePicker bind:active={theme} />

<header class="hero">
  <p class="eyebrow">ANSI X9.24 · ISO 9564 · Rust → WebAssembly</p>
  <h1>paycrypt</h1>
  <p class="lede">
    An interactive explainer for payments cryptography. It runs the
    <strong>real compiled paycrypt library</strong> in your browser via WebAssembly
    and shows the intermediate derivation state that ordinary calculators hide.
  </p>
  <p class="ver">{version()}</p>
  <FlowStrip />
</header>

<DukptLadder />
<PinBlockStepper />

<footer>
  <p>
    Educational / validation use only. Not for production, not PCI-compliant,
    not audited. Every value is computed by the real library from synthetic test
    data.
  </p>
</footer>

<style>
  .hero {
    padding: 1rem 0 0.5rem;
  }
  .hero h1 {
    font-size: 2.6rem;
    margin: 0.1rem 0 0.6rem;
    background: linear-gradient(90deg, #ffffff, #86b6ef);
    -webkit-background-clip: text;
    background-clip: text;
    color: transparent;
  }
  .lede {
    color: var(--text-secondary);
    font-size: 1.05rem;
    max-width: 60ch;
  }
  .ver {
    font-family: var(--mono);
    font-size: 0.75rem;
    color: var(--text-muted);
    margin-top: 0.8rem;
  }
  footer {
    margin-top: 2.5rem;
    padding-top: 1.2rem;
    border-top: 1px solid var(--border);
    color: var(--text-muted);
    font-size: 0.82rem;
    max-width: 66ch;
  }
</style>
