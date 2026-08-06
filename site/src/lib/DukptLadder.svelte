<script lang="ts">
  import { tdes_ladder_steps } from "../../../crates/paycrypt-wasm/pkg/paycrypt_wasm.js";
  import ByteGrid from "./ByteGrid.svelte";
  import KsnBar from "./KsnBar.svelte";
  import HexValue from "./HexValue.svelte";

  type Step = { label: string; bytes: number[]; hex: string; note: string };

  // Synthetic demo values (the canonical published test vector).
  let bdk = "0123456789ABCDEFFEDCBA9876543210";
  let counter = 3;
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
  $: popcount = counter.toString(2).split("").filter((x) => x === "1").length;
</script>

<section class="panel">
  <p class="eyebrow">DUKPT key ladder · TDES · ANSI X9.24-1</p>
  <h2>One transaction, one key</h2>
  <span class="demo-tag">demo data: never enter real keys</span>

  <p class="note">
    The Base Derivation Key never changes. Each set bit of the transaction
    counter runs one non-reversible derivation, so every transaction lands on a
    different key, and the acquirer re-derives the same key from the KSN alone.
    <strong>Every intermediate key below is normally invisible.</strong>
  </p>

  <label class="field">
    <span>Base Derivation Key (BDK)</span>
    <input bind:value={bdk} spellcheck="false" />
  </label>

  <label class="field">
    <span>transaction counter: {counter} · {popcount} set {popcount === 1 ? "bit" : "bits"} → {popcount} ladder {popcount === 1 ? "derivation" : "derivations"}</span>
    <input type="range" min="1" max="255" bind:value={counter} aria-label="transaction counter" />
  </label>

  <KsnBar {counter} />

  <label class="toggle"><input type="checkbox" bind:checked={binary} /> show bytes in binary</label>

  {#if steps.length === 0}
    <p class="err">Invalid BDK or KSN.</p>
  {:else}
    <ol class="ladder">
      {#each steps as step, i}
        {@const isLast = i === steps.length - 1}
        <li class:result={isLast}>
          <div class="rail" aria-hidden="true">
            <span class="dot" class:result={isLast}></span>
            {#if !isLast}<span class="line"></span>{/if}
          </div>
          <div class="body">
            <div class="head">
              <strong>{step.label}</strong>
              <span class="sub">{step.note}</span>
            </div>
            <ByteGrid
              bytes={new Uint8Array(step.bytes)}
              prev={i > 0 ? new Uint8Array(steps[i - 1].bytes) : null}
              {binary}
            />
            <HexValue value={step.hex} />
          </div>
        </li>
      {/each}
    </ol>
  {/if}
</section>

<style>
  .ladder {
    list-style: none;
    padding: 0;
    margin: 1.1rem 0 0;
  }
  .ladder li {
    display: grid;
    grid-template-columns: 1.4rem 1fr;
    gap: 0.9rem;
  }
  .rail {
    display: flex;
    flex-direction: column;
    align-items: center;
  }
  .dot {
    width: 12px;
    height: 12px;
    border-radius: 50%;
    background: var(--border-strong);
    margin-top: 0.35rem;
    flex: 0 0 auto;
  }
  .dot.result {
    background: var(--brand);
    box-shadow: 0 0 0 4px rgba(40, 87, 214, 0.18);
  }
  .line {
    width: 2px;
    flex: 1 1 auto;
    background: var(--border-strong);
    margin: 4px 0;
  }
  .body {
    padding-bottom: 1.4rem;
    min-width: 0;
  }
  .head {
    margin-bottom: 0.55rem;
  }
  .head strong {
    font-size: 1rem;
    color: var(--ink);
  }
  .head .sub {
    display: block;
    color: var(--ink-3);
    font-size: 0.85rem;
  }
  .result .head strong {
    color: var(--brand);
  }
</style>
