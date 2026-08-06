<script lang="ts">
  // Changed bytes are marked with text as well as colour.
  let {
    bytes,
    prev = null,
    binary = false,
  }: { bytes: Uint8Array; prev?: Uint8Array | null; binary?: boolean } = $props();

  function changed(i: number): boolean {
    return prev != null && prev[i] !== bytes[i];
  }
  function render(b: number): string {
    return binary ? b.toString(2).padStart(8, "0") : b.toString(16).toUpperCase().padStart(2, "0");
  }
</script>

<div class="grid" role="list">
  {#each Array.from(bytes) as b, i}
    <span class="cell" class:changed={changed(i)} role="listitem"
          aria-label={`byte ${i}${changed(i) ? " (changed)" : ""}`}>
      {render(b)}{#if changed(i)}<span class="mark" aria-hidden="true">*</span>{/if}
    </span>
  {/each}
</div>

<style>
  .grid { display: flex; flex-wrap: wrap; gap: 0.25rem; font-family: ui-monospace, monospace; }
  .cell { padding: 0.15rem 0.35rem; border-radius: 3px; background: #f0f0f0; }
  .cell.changed { background: #ffe08a; font-weight: 700; }
  .mark { margin-left: 1px; }
</style>
