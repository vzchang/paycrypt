<script lang="ts">
  import { tdes_ladder_steps } from "../../../crates/paycrypt-wasm/pkg/paycrypt_wasm.js";
  import ByteGrid from "./ByteGrid.svelte";

  type Step = { label: string; bytes: number[]; hex: string; note: string };

  // Synthetic demo values (the canonical published test vector).
  let bdk = "0123456789ABCDEFFEDCBA9876543210";
  let counter = 3;
  let stepIndex = 0;
  let binary = false;

  function ksnHex(c: number): string {
    // IKSN FFFF9876543210E0 0000 with the 21-bit counter in the low bits.
    return "FFFF9876543210E0" + c.toString(16).toUpperCase().padStart(4, "0");
  }

  let steps: Step[] = [];
  $: {
    try {
      steps = tdes_ladder_steps(bdk, ksnHex(counter)) as Step[];
    } catch {
      steps = [];
    }
  }
  $: if (stepIndex >= steps.length) stepIndex = Math.max(0, steps.length - 1);
  $: current = steps[stepIndex];
  $: prev = stepIndex > 0 ? new Uint8Array(steps[stepIndex - 1].bytes) : null;
</script>

<section>
  <h2>DUKPT key ladder (TDES, ANSI X9.24-1)</h2>
  <p class="demo">Demo data only: never enter real keys or PANs.</p>

  <label>BDK <input bind:value={bdk} size="34" spellcheck="false" /></label>
  <label>
    transaction counter: {counter}
    <input type="range" min="1" max="255" bind:value={counter}
           aria-label="transaction counter" />
  </label>
  <label><input type="checkbox" bind:checked={binary} /> show binary</label>

  {#if steps.length === 0}
    <p class="err">Invalid BDK or KSN.</p>
  {:else}
    <div class="stepper" role="group" aria-label="ladder step controls">
      <button on:click={() => (stepIndex = Math.max(0, stepIndex - 1))}
              disabled={stepIndex === 0}>◀ Prev</button>
      <span>step {stepIndex + 1} / {steps.length}: <strong>{current.label}</strong></span>
      <button on:click={() => (stepIndex = Math.min(steps.length - 1, stepIndex + 1))}
              disabled={stepIndex === steps.length - 1}>Next ▶</button>
    </div>
    <p class="note">{current.note}</p>
    <ByteGrid bytes={new Uint8Array(current.bytes)} {prev} {binary} />
    <p class="hex">{current.hex}</p>
  {/if}
</section>

<style>
  section { max-width: 640px; }
  .demo { color: #a15; font-size: 0.85rem; }
  label { display: block; margin: 0.5rem 0; }
  .stepper { display: flex; gap: 1rem; align-items: center; margin: 0.75rem 0; }
  .note { color: #555; }
  .hex { font-family: ui-monospace, monospace; color: #333; }
  .err { color: #a00; }
</style>
