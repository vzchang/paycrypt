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
  $: popcount = counter.toString(2).split("").filter((x) => x === "1").length;
</script>

<section class="panel">
  <p class="eyebrow">DUKPT key ladder · TDES · ANSI X9.24-1</p>
  <h2>One transaction, one key</h2>
  <span class="demo-tag">demo data: never enter real keys</span>

  <p class="note">
    The Base Derivation Key never changes. Each set bit of the transaction
    counter runs one non-reversible derivation, so every transaction lands on a
    different key while the acquirer can re-derive it from the KSN alone.
  </p>

  <label class="field">
    <span>Base Derivation Key (BDK)</span>
    <input bind:value={bdk} spellcheck="false" />
  </label>

  <label class="field">
    <span>transaction counter: {counter} ({popcount} set {popcount === 1 ? "bit" : "bits"} → {popcount} ladder {popcount === 1 ? "step" : "steps"})</span>
    <input type="range" min="1" max="255" bind:value={counter} aria-label="transaction counter" />
  </label>

  <label class="toggle"><input type="checkbox" bind:checked={binary} /> show binary</label>

  {#if steps.length === 0}
    <p class="err">Invalid BDK or KSN.</p>
  {:else}
    <div class="stepper" role="group" aria-label="ladder step controls">
      <button on:click={() => (stepIndex = Math.max(0, stepIndex - 1))} disabled={stepIndex === 0}>◀ Prev</button>
      <span class="pos">step {stepIndex + 1} / {steps.length}: <strong>{current.label}</strong></span>
      <button on:click={() => (stepIndex = Math.min(steps.length - 1, stepIndex + 1))} disabled={stepIndex === steps.length - 1}>Next ▶</button>
    </div>
    <p class="note">{current.note}</p>
    <ByteGrid bytes={new Uint8Array(current.bytes)} {prev} {binary} />
    <p class="hex">{current.hex}</p>
  {/if}
</section>
