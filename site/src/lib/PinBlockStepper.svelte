<script lang="ts">
  import { iso0_steps } from "../../../crates/paycrypt-wasm/pkg/paycrypt_wasm.js";
  import ByteGrid from "./ByteGrid.svelte";

  type Step = { label: string; bytes: number[]; hex: string; note: string };

  // Synthetic demo values (the canonical psec ISO-0 vector).
  let pin = "1234";
  let pan = "5555555551234567";
  let binary = false;

  let steps: Step[] = [];
  $: {
    try {
      steps = iso0_steps(pin, pan) as Step[];
    } catch {
      steps = [];
    }
  }
</script>

<section>
  <h2>ISO 9564 format-0 PIN block</h2>
  <p class="demo">Demo data only: never enter real PINs or PANs.</p>

  <label>PIN <input bind:value={pin} size="14" spellcheck="false" /></label>
  <label>PAN <input bind:value={pan} size="22" spellcheck="false" /></label>
  <label><input type="checkbox" bind:checked={binary} /> show binary</label>

  {#if steps.length === 0}
    <p class="err">Invalid PIN or PAN.</p>
  {:else}
    <ol class="frames">
      {#each steps as step, i}
        <li>
          <div class="head"><strong>{step.label}</strong>: {step.note}</div>
          <ByteGrid
            bytes={new Uint8Array(step.bytes)}
            prev={i > 0 ? new Uint8Array(steps[i - 1].bytes) : null}
            {binary}
          />
          <p class="hex">{step.hex}</p>
        </li>
      {/each}
    </ol>
  {/if}
</section>

<style>
  section { max-width: 640px; }
  .demo { color: #a15; font-size: 0.85rem; }
  label { display: block; margin: 0.5rem 0; }
  .frames { list-style: none; padding: 0; }
  .frames li { margin: 0.75rem 0; padding: 0.5rem; border-left: 3px solid #ccd; }
  .head { color: #555; margin-bottom: 0.35rem; }
  .hex { font-family: ui-monospace, monospace; color: #333; }
  .err { color: #a00; }
</style>
