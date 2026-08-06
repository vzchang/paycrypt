<script lang="ts">
  // Changed bytes get a ring and an aria marker, never colour alone.
  export let bytes: Uint8Array;
  export let prev: Uint8Array | null = null;
  export let binary = false;

  function changed(i: number): boolean {
    return prev != null && prev[i] !== bytes[i];
  }
  function render(b: number): string {
    return binary ? b.toString(2).padStart(8, "0") : b.toString(16).toUpperCase().padStart(2, "0");
  }
</script>

<div class="grid" role="list" class:binary>
  {#each Array.from(bytes) as b, i}
    <span
      class="cell"
      class:changed={changed(i)}
      role="listitem"
      aria-label={`byte ${i}${changed(i) ? " changed" : ""}`}
    >
      <span class="val">{render(b)}</span>
    </span>
  {/each}
</div>

<style>
  .grid {
    display: flex;
    flex-wrap: wrap;
    gap: 2px;
    font-family: var(--mono);
  }
  .cell {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    min-width: 2.6ch;
    padding: 0.4rem 0.4rem;
    background: var(--surface-2);
    border-radius: 5px;
    border: 1px solid var(--border);
  }
  .binary .cell {
    min-width: 9ch;
  }
  .cell .val {
    font-size: 0.95rem;
    color: var(--text-primary);
    letter-spacing: 0.02em;
  }
  .cell.changed {
    box-shadow: 0 0 0 2px var(--accent);
    border-color: var(--accent);
    animation: pop 0.25s ease-out;
  }
  .cell.changed .val {
    font-weight: 700;
    color: var(--accent-bright);
  }
  @keyframes pop {
    from {
      transform: scale(0.9);
      box-shadow: 0 0 0 4px rgba(212, 175, 106, 0.5);
    }
    to {
      transform: scale(1);
      box-shadow: 0 0 0 2px var(--accent);
    }
  }
  @media (prefers-reduced-motion: reduce) {
    .cell.changed {
      animation: none;
    }
  }
</style>
