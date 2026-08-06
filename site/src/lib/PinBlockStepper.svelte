<script lang="ts">
  import { iso0_steps } from "../../../crates/paycrypt-wasm/pkg/paycrypt_wasm.js";
  import ByteGrid from "./ByteGrid.svelte";
  import HexValue from "./HexValue.svelte";
  import { safeSteps, type Step } from "./dukpt";

  // Synthetic demo values (the canonical psec ISO-0 vector).
  let pin = "1234";
  let pan = "5555555551234567";
  let binary = false;

  let steps: Step[] = [];
  $: steps = safeSteps(() => iso0_steps(pin, pan));
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
    padding: 0.9rem 1rem;
    background: var(--surface-2);
    border-radius: 6px;
    border-left: 3px solid var(--border-strong);
  }
  .frames li.result {
    border-left-color: var(--accent);
  }
  .head {
    display: flex;
    gap: 0.7rem;
    align-items: baseline;
    margin-bottom: 0.6rem;
  }
  .idx {
    font-family: var(--mono);
    font-size: 0.72rem;
    font-weight: 700;
    color: var(--surface-0);
    background: var(--text-muted);
    border-radius: 4px;
    padding: 0.08rem 0.45rem;
  }
  .result .idx {
    background: var(--accent);
    color: var(--surface-0);
  }
  .sub {
    display: block;
    color: var(--text-muted);
    font-size: 0.8rem;
  }
</style>
