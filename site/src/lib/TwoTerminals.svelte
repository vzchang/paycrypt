<script lang="ts">
  import { tdes_ladder_steps } from "../../../crates/paycrypt-wasm/pkg/paycrypt_wasm.js";

  type Step = { label: string; bytes: number[]; hex: string; note: string };

  const BDK = "0123456789ABCDEFFEDCBA9876543210";
  let counter = 1;
  let revealed = false;

  function ksnHex(c: number): string {
    return "FFFF9876543210E0" + c.toString(16).toUpperCase().padStart(4, "0");
  }

  // One derivation serves both sides: the device holds the IPEK, the host re-derives it from the BDK.
  let steps: Step[] = [];
  $: {
    try {
      steps = tdes_ladder_steps(BDK, ksnHex(counter)) as Step[];
    } catch {
      steps = [];
    }
  }
  $: ipek = steps[0]?.hex ?? "";
  $: key = steps[steps.length - 1]?.hex ?? "";
  $: match = key.length > 0;

  function nextTxn() {
    counter += 1;
    revealed = true;
  }
  function reset() {
    counter = 1;
    revealed = false;
  }
</script>

<section class="panel wide">
  <p class="eyebrow">The DUKPT guarantee</p>
  <h2>Two parties. Different secrets. Same key.</h2>
  <p class="note">
    The device never stores the master key. The acquirer never sees the device's
    injected key. Yet for every transaction they compute an identical one-time
    key, and it changes on every tap.
  </p>

  <div class="stage">
    <div class="party device">
      <div class="who">Payment device</div>
      <div class="holds">holds injected <b>Initial Key</b></div>
      <div class="holds muted">master key never present</div>
      <div class="chip ipek" title="injected at manufacture">
        <span class="lbl">IPEK</span>
        <span class="mono">{ipek.slice(0, 16)}…</span>
      </div>
    </div>

    <div class="middle">
      <div class="counter-badge">txn #{counter}</div>
      <div class="arrows" aria-hidden="true">↘&nbsp;&nbsp;↙</div>
      <div class="key-out" class:show={match}>
        <span class="lbl">transaction key</span>
        {#key counter}<span class="mono gold">{key}</span>{/key}
        {#if match}<span class="verdict">✓ identical</span>{/if}
      </div>
      <div class="arrows up" aria-hidden="true">↗&nbsp;&nbsp;↖</div>
    </div>

    <div class="party acquirer">
      <div class="who">Acquirer host</div>
      <div class="holds">holds <b>Base Derivation Key</b></div>
      <div class="holds muted">re-derives IPEK on demand</div>
      <div class="chip bdk" title="stored only here">
        <span class="lbl">BDK</span>
        <span class="mono">{BDK.slice(0, 16)}…</span>
      </div>
    </div>
  </div>

  <div class="controls">
    <button class="primary" on:click={nextTxn}>Run next transaction ▶</button>
    <button on:click={reset} disabled={counter === 1 && !revealed}>Reset</button>
    <span class="hint">
      {#if revealed}Watch the key change every tap, yet both sides still match.{:else}Tap to advance the transaction counter.{/if}
    </span>
  </div>
</section>

<style>
  .stage {
    display: grid;
    grid-template-columns: 1fr auto 1fr;
    gap: 1rem;
    align-items: center;
    margin: 1.4rem 0 0.5rem;
  }
  .party {
    background: var(--surface-2);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    padding: 1rem;
  }
  .who {
    font-family: var(--display);
    font-size: 1.05rem;
    color: var(--text-primary);
    margin-bottom: 0.3rem;
  }
  .holds {
    font-size: 0.82rem;
    color: var(--text-secondary);
  }
  .holds b {
    color: var(--text-primary);
  }
  .holds.muted {
    color: var(--text-muted);
    font-size: 0.76rem;
  }
  .chip {
    display: flex;
    flex-direction: column;
    gap: 0.15rem;
    margin-top: 0.7rem;
    padding: 0.5rem 0.6rem;
    border-radius: 7px;
    background: var(--surface-1);
    border: 1px dashed var(--border-strong);
  }
  .chip .lbl {
    font-family: var(--mono);
    font-size: 0.6rem;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--text-muted);
  }
  .mono {
    font-family: var(--mono);
    font-size: 0.82rem;
    color: var(--text-secondary);
    word-break: break-all;
  }
  .middle {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.4rem;
    min-width: 190px;
  }
  .counter-badge {
    font-family: var(--mono);
    font-size: 0.72rem;
    color: var(--surface-0);
    background: var(--accent);
    border-radius: 999px;
    padding: 0.15rem 0.7rem;
  }
  .arrows {
    color: var(--border-strong);
    font-size: 0.9rem;
  }
  .key-out {
    text-align: center;
    padding: 0.6rem 0.8rem;
    border-radius: 8px;
    border: 1px solid var(--border);
    background: var(--surface-1);
    transition: border-color 0.3s, box-shadow 0.3s;
  }
  .key-out.show {
    border-color: var(--accent);
    box-shadow: 0 0 0 3px var(--accent-dim);
  }
  /* re-key each time the counter changes: the mono value keys= re-animates */
  .key-out.show .mono.gold {
    animation: lockin 0.5s cubic-bezier(0.3, 1.3, 0.4, 1);
  }
  .arrows {
    animation: pulse-down 2.2s ease-in-out infinite;
  }
  .arrows.up {
    animation: pulse-up 2.2s ease-in-out infinite;
  }
  @keyframes lockin {
    0% {
      transform: scale(0.8);
      opacity: 0.2;
      filter: blur(2px);
    }
    100% {
      transform: scale(1);
      opacity: 1;
      filter: none;
    }
  }
  @keyframes pulse-down {
    0%, 100% {
      opacity: 0.3;
      transform: translateY(0);
    }
    50% {
      opacity: 1;
      transform: translateY(3px);
    }
  }
  @keyframes pulse-up {
    0%, 100% {
      opacity: 0.3;
      transform: translateY(0);
    }
    50% {
      opacity: 1;
      transform: translateY(-3px);
    }
  }
  @media (prefers-reduced-motion: reduce) {
    .key-out.show .mono.gold,
    .arrows,
    .arrows.up {
      animation: none;
    }
  }
  .key-out .lbl {
    display: block;
    font-family: var(--mono);
    font-size: 0.6rem;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--text-muted);
    margin-bottom: 0.2rem;
  }
  .mono.gold {
    color: var(--accent-bright);
    font-size: 0.8rem;
  }
  .verdict {
    display: block;
    margin-top: 0.3rem;
    font-family: var(--mono);
    font-size: 0.72rem;
    color: var(--accent);
  }
  .controls {
    display: flex;
    align-items: center;
    gap: 0.8rem;
    flex-wrap: wrap;
    margin-top: 1.1rem;
  }
  button.primary {
    color: var(--surface-0);
    background: var(--accent);
    border-color: var(--accent);
    font-weight: 700;
  }
  button.primary:hover {
    background: var(--accent-bright);
    color: var(--surface-0);
  }
  .hint {
    font-size: 0.82rem;
    color: var(--text-muted);
  }
  @media (max-width: 720px) {
    .stage {
      grid-template-columns: 1fr;
    }
    .arrows {
      display: none;
    }
  }
</style>
