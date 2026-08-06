<script lang="ts">
  import { iso0_steps } from "../../../crates/paycrypt-wasm/pkg/paycrypt_wasm.js";
  import ByteGrid from "./ByteGrid.svelte";
  import HexValue from "./HexValue.svelte";

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

<section class="panel">
  <p class="eyebrow">PIN block · ISO 9564 format 0 (ANSI X9.8)</p>
  <h2>Binding the PIN to the account</h2>
  <span class="demo-tag">demo data: never enter real PINs or PANs</span>

  <p class="note">
    The PIN field and an account field derived from the PAN are XORed together,
    so the same PIN on a different account never produces the same block.
  </p>

  <label class="field"><span>PIN</span><input bind:value={pin} spellcheck="false" /></label>
  <label class="field"><span>PAN</span><input bind:value={pan} spellcheck="false" /></label>
  <label class="toggle"><input type="checkbox" bind:checked={binary} /> show binary</label>

  {#if steps.length === 0}
    <p class="err">Invalid PIN or PAN.</p>
  {:else}
    <ol class="frames">
      {#each steps as step, i}
        <li class:result={i === steps.length - 1}>
          <div class="head">
            <span class="idx">{i + 1}</span>
            <div>
              <strong>{step.label}</strong>
              <span class="sub">{step.note}</span>
            </div>
          </div>
          <ByteGrid
            bytes={new Uint8Array(step.bytes)}
            prev={i > 0 ? new Uint8Array(steps[i - 1].bytes) : null}
            {binary}
          />
          <HexValue value={step.hex} />
        </li>
      {/each}
    </ol>
  {/if}
</section>

<style>
  .frames {
    list-style: none;
    padding: 0;
    margin: 1rem 0 0;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  .frames li {
    padding: 1rem 1.1rem;
    background: var(--surface-alt);
    border: 1px solid var(--border);
    border-radius: 8px;
    border-left: 3px solid var(--border-strong);
  }
  .frames li.result {
    border-left-color: var(--brand);
    background: var(--brand-weak);
  }
  .head {
    display: flex;
    gap: 0.7rem;
    align-items: baseline;
    margin-bottom: 0.65rem;
  }
  .idx {
    font-family: var(--mono);
    font-size: 0.78rem;
    font-weight: 700;
    color: #fff;
    background: var(--ink-3);
    border-radius: 5px;
    padding: 0.1rem 0.5rem;
  }
  .result .idx {
    background: var(--brand);
  }
  .head strong {
    color: var(--ink);
  }
  .sub {
    display: block;
    color: var(--ink-3);
    font-size: 0.85rem;
  }
</style>
