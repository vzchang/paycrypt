<script lang="ts">
  import "./app.css";
  import { version } from "../../crates/paycrypt-wasm/pkg/paycrypt_wasm.js";
  import DukptLadder from "./lib/DukptLadder.svelte";
  import PinBlockStepper from "./lib/PinBlockStepper.svelte";
  import FlowStrip from "./lib/FlowStrip.svelte";
</script>

<header class="hero">
  <p class="eyebrow">ANSI X9.24 · ISO 9564 · Rust → WebAssembly</p>
  <h1>How card payments are secured</h1>
  <p class="lede">
    An interactive walkthrough of the cryptography behind a card transaction. It
    runs the <strong>real compiled paycrypt library</strong> in your browser, so
    every value below is computed live, not a mock-up.
  </p>
  <FlowStrip />
</header>

<main>
  <DukptLadder />
  <PinBlockStepper />
</main>

<footer>
  <p>
    Educational use only. Not for production, not PCI-compliant, not audited.
    All values are derived from synthetic test data.
  </p>
  <p class="ver">{version()}</p>
</footer>

<style>
  .hero {
    padding: 0.5rem 0 0.5rem;
  }
  .hero h1 {
    font-size: 2.4rem;
    margin: 0.2rem 0 0.7rem;
  }
  .lede {
    color: var(--ink-2);
    font-size: 1.15rem;
    max-width: 58ch;
  }
  footer {
    margin-top: 3rem;
    padding-top: 1.4rem;
    border-top: 1px solid var(--border);
    color: var(--ink-3);
    font-size: 0.88rem;
  }
  .ver {
    font-family: var(--mono);
    font-size: 0.78rem;
    color: var(--ink-3);
    margin-top: 0.4rem;
  }
</style>
