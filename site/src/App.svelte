<script lang="ts">
  import { version } from "../../crates/paycrypt-wasm/pkg/paycrypt_wasm.js";
  import DukptLadder from "./lib/DukptLadder.svelte";
  import PinBlockStepper from "./lib/PinBlockStepper.svelte";
</script>

<header>
  <h1>paycrypt</h1>
  <p>An interactive explainer that runs the real compiled paycrypt library
     (WebAssembly) and shows the intermediate derivation state every existing
     calculator hides.</p>
  <p class="ver">{version()}</p>
</header>

<DukptLadder />
<PinBlockStepper />

<footer>
  <p>Educational / validation use only. Not for production, not PCI-compliant,
     not audited. Synthetic test data only.</p>
</footer>

<style>
  :global(body) { font-family: system-ui, sans-serif; margin: 2rem; line-height: 1.5; }
  header { max-width: 640px; }
  .ver { color: #888; font-family: ui-monospace, monospace; font-size: 0.8rem; }
  footer { margin-top: 2rem; color: #777; font-size: 0.85rem; max-width: 640px; }
</style>
