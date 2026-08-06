<script lang="ts">
  import { iso0_steps } from "../../../crates/paycrypt-wasm/pkg/paycrypt_wasm.js";
  import { safeSteps, type Step } from "./dukpt";

  let pin = "1234";
  let pan = "5555555551234567";
  let hover: number | null = null;

  let steps: Step[] = [];
  $: steps = safeSteps(() => iso0_steps(pin, pan));
  // steps: [PIN field, account field, XOR result]
  $: pinField = steps[0]?.bytes ?? [];
  $: acctField = steps[1]?.bytes ?? [];
  $: result = steps[2]?.bytes ?? [];
  $: nibbles = toNibbles(pinField, acctField, result);

  function toNibbles(a: number[], b: number[], c: number[]) {
    const out: { p: number; n: number; r: number }[] = [];
    for (let i = 0; i < a.length; i++) {
      out.push({ p: a[i] >> 4, n: b[i] >> 4, r: c[i] >> 4 });
      out.push({ p: a[i] & 0xf, n: b[i] & 0xf, r: c[i] & 0xf });
    }
    return out;
  }
  const hx = (v: number) => v.toString(16).toUpperCase();
  const bn = (v: number) => v.toString(2).padStart(4, "0");
</script>

<section class="panel">
  <p class="eyebrow">Interactive · XOR</p>
  <h2>Why the same PIN never repeats</h2>
  <p class="note">
    The PIN field is XORed with an account field built from the PAN. Hover any
    column to see the bit math. Change the PAN by one digit and the whole block
    changes. That's the property that stops a stolen block being replayed on
    another card.
  </p>

  <div class="io">
    <label class="field"><span>PIN</span><input bind:value={pin} spellcheck="false" /></label>
    <label class="field"><span>PAN</span><input bind:value={pan} spellcheck="false" /></label>
  </div>

  {#if nibbles.length === 0}
    <p class="err">Invalid PIN or PAN.</p>
  {:else}
    <div class="rows" role="grid" aria-label="XOR of PIN field and account field">
      <div class="rowlabel">PIN field</div>
      <div class="row">
        {#each nibbles as nb, i}
          <button class="nib pin" class:hot={hover === i}
                  on:mouseenter={() => (hover = i)} on:mouseleave={() => (hover = null)}
                  on:focus={() => (hover = i)} on:blur={() => (hover = null)}
                  aria-label={`PIN nibble ${i}: hex ${hx(nb.p)}`}>{hx(nb.p)}</button>
        {/each}
      </div>

      <div class="rowlabel op">XOR ⊕</div>
      <div class="row">
        {#each nibbles as nb, i}
          <button class="nib pan" class:hot={hover === i}
                  on:mouseenter={() => (hover = i)} on:mouseleave={() => (hover = null)}
                  on:focus={() => (hover = i)} on:blur={() => (hover = null)}
                  aria-label={`account nibble ${i}: hex ${hx(nb.n)}`}>{hx(nb.n)}</button>
        {/each}
      </div>

      <div class="rowlabel eq">= block</div>
      <div class="row">
        {#each nibbles as nb, i}
          <span class="nib res" class:hot={hover === i}>{hx(nb.r)}</span>
        {/each}
      </div>
    </div>

    <div class="readout" aria-live="polite">
      {#if hover !== null}
        <span class="mono">
          {bn(nibbles[hover].p)} ⊕ {bn(nibbles[hover].n)} = <b>{bn(nibbles[hover].r)}</b>
          &nbsp;·&nbsp; {hx(nibbles[hover].p)} ⊕ {hx(nibbles[hover].n)} = <b>{hx(nibbles[hover].r)}</b>
        </span>
      {:else}
        <span class="muted">Hover a column to see the bitwise XOR.</span>
      {/if}
    </div>
  {/if}
</section>

<style>
  .io {
    display: flex;
    gap: 1rem;
    flex-wrap: wrap;
  }
  .io .field {
    flex: 1 1 160px;
  }
  .rows {
    display: grid;
    grid-template-columns: auto 1fr;
    gap: 0.4rem 0.8rem;
    align-items: center;
    margin: 1.2rem 0 0.8rem;
  }
  .rowlabel {
    font-family: var(--mono);
    font-size: 0.72rem;
    color: var(--text-muted);
    text-align: right;
    white-space: nowrap;
  }
  .rowlabel.op {
    color: var(--accent);
  }
  .row {
    display: flex;
    gap: 3px;
    flex-wrap: nowrap;
    overflow-x: auto;
  }
  .nib {
    font-family: var(--mono);
    font-size: 0.9rem;
    width: 1.9ch;
    min-width: 1.9ch;
    text-align: center;
    padding: 0.35rem 0;
    border-radius: 4px;
    border: 1px solid var(--border);
    background: var(--surface-2);
    color: var(--text-secondary);
    cursor: default;
    line-height: 1;
  }
  button.nib {
    cursor: pointer;
  }
  .nib.pin.hot {
    border-color: var(--role-control);
    color: var(--text-primary);
  }
  .nib.pan.hot {
    border-color: var(--role-pan);
    color: var(--text-primary);
  }
  .nib.res {
    background: var(--surface-1);
  }
  .nib {
    transition: transform 0.12s ease, border-color 0.12s ease, background 0.12s ease;
  }
  .nib.res.hot {
    border-color: var(--accent);
    background: var(--accent-dim);
    color: var(--accent-bright);
    font-weight: 700;
    transform: scale(1.25);
  }
  .nib.pin.hot,
  .nib.pan.hot {
    transform: scale(1.15);
  }
  @media (prefers-reduced-motion: reduce) {
    .nib {
      transition: none;
    }
    .nib.hot {
      transform: none;
    }
  }
  .readout {
    min-height: 1.4rem;
    font-size: 0.9rem;
  }
  .readout .mono {
    font-family: var(--mono);
    color: var(--text-secondary);
  }
  .readout .mono b {
    color: var(--accent-bright);
  }
  .readout .muted {
    color: var(--text-muted);
  }
</style>
