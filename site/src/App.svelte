<script lang="ts">
  import "./app.css";
  import { version } from "../../crates/paycrypt-wasm/pkg/paycrypt_wasm.js";
  import DukptLadder from "./lib/DukptLadder.svelte";
  import PinBlockStepper from "./lib/PinBlockStepper.svelte";
  import FlowStrip from "./lib/FlowStrip.svelte";
  import TwoTerminals from "./lib/TwoTerminals.svelte";
  import XorPlayground from "./lib/XorPlayground.svelte";
</script>

<header class="hero">
  <p class="eyebrow">ANSI X9.24 · ISO 9564</p>
  <h1>Every swipe mints<br /><span class="gold">a key that dies once.</span></h1>
  <p class="lede">
    Card payments are protected by cryptography that runs in the half-second
    between tapping your card and the "approved" beep. This page performs that
    cryptography live in your browser (the real
    <strong>paycrypt</strong> library, compiled to WebAssembly) and shows you the
    working most tools keep hidden.
  </p>
  <FlowStrip />
</header>

<TwoTerminals />

<main class="grid">
  <DukptLadder />
  <PinBlockStepper />
</main>

<XorPlayground />

<footer>
  <p>
    Educational use only; not for production, not PCI-compliant, not audited.
    Every value is derived from synthetic test data by the real library.
  </p>
  <p class="ver">{version()}</p>
</footer>

<style>
  .hero {
    padding: 0.5rem 0 0.75rem;
  }
  .hero h1 {
    font-size: clamp(2.2rem, 6vw, 3.4rem);
    margin: 0.3rem 0 1rem;
  }
  .hero .gold {
    color: var(--accent);
    font-style: italic;
  }
  .lede {
    color: var(--text-secondary);
    font-size: 1.12rem;
    max-width: 56ch;
  }
  .grid {
    display: grid;
    grid-template-columns: 1fr;
    gap: 1.5rem;
    align-items: start;
  }
  .grid :global(.panel) {
    margin: 0;
  }
  @media (min-width: 900px) {
    .grid {
      grid-template-columns: 1fr 1fr;
    }
    /* the right column never touches the spine, so its node would float in the gap */
    .grid :global(.panel:nth-child(2)::before) {
      display: none;
    }
  }
  footer {
    margin-top: 3rem;
    padding-top: 1.4rem;
    border-top: 1px solid var(--border);
    color: var(--text-muted);
    font-size: 0.84rem;
  }
  .ver {
    font-family: var(--mono);
    font-size: 0.76rem;
    color: var(--text-muted);
    margin-top: 0.4rem;
  }
</style>
