<script lang="ts">
  export let counter: number;
  export let activeBit: number | null = null; // 0-based from MSB

  $: bits = Array.from({ length: 21 }, (_, i) => {
    const bitPos = 20 - i;
    return { set: (counter & (1 << bitPos)) !== 0, pos: bitPos, idx: i };
  });
</script>

<div class="ksn" role="img" aria-label={`transaction counter ${counter}, ${counter.toString(2)} binary`}>
  <div class="bits">
    {#each bits as bit}
      <span
        class="bit"
        class:set={bit.set}
        class:active={activeBit === bit.idx}
        title={`bit ${bit.pos}`}
      >{bit.set ? "1" : "0"}</span>
    {/each}
  </div>
  <div class="caption">
    <span>21-bit transaction counter</span>
    <span class="hint">each <b>1</b> = one ladder derivation</span>
  </div>
</div>

<style>
  .ksn {
    margin: 0.6rem 0 0.2rem;
  }
  .bits {
    display: flex;
    gap: 2px;
    flex-wrap: wrap;
  }
  .bit {
    font-family: var(--mono);
    font-size: 0.72rem;
    width: 1.6ch;
    text-align: center;
    padding: 0.3rem 0;
    border-radius: 4px;
    background: var(--surface-alt);
    border: 1px solid var(--border);
    color: var(--ink-3);
    transition: background 0.2s, color 0.2s, box-shadow 0.2s;
  }
  .bit.set {
    background: var(--brand);
    border-color: var(--brand);
    color: #fff;
    font-weight: 700;
  }
  .bit.active {
    box-shadow: 0 0 0 2px var(--role-pad);
  }
  .caption {
    display: flex;
    justify-content: space-between;
    flex-wrap: wrap;
    gap: 0.5rem;
    font-size: 0.78rem;
    color: var(--ink-3);
    margin-top: 0.45rem;
  }
  .caption .hint b {
    color: var(--brand);
  }
  @media (prefers-reduced-motion: reduce) {
    .bit {
      transition: none;
    }
  }
</style>
