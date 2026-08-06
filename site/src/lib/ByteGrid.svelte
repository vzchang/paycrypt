<script lang="ts">
  // Changed bytes get a ring and an aria marker, never colour alone.
  // `roles` colour cells by nibble role; the role name is always shown too.
  export let bytes: Uint8Array;
  export let prev: Uint8Array | null = null;
  export let binary = false;
  export let roles: string[] | null = null;

  const ROLE_VAR: Record<string, string> = {
    control: "var(--role-control)",
    length: "var(--role-length)",
    pin: "var(--role-pin)",
    pad: "var(--role-pad)",
    pan: "var(--role-pan)",
  };

  function changed(i: number): boolean {
    return prev != null && prev[i] !== bytes[i];
  }
  function render(b: number): string {
    return binary ? b.toString(2).padStart(8, "0") : b.toString(16).toUpperCase().padStart(2, "0");
  }
  function tint(i: number): string {
    const r = roles?.[i];
    return r && ROLE_VAR[r] ? ROLE_VAR[r] : "transparent";
  }
</script>

<div class="grid" role="list" class:binary>
  {#each Array.from(bytes) as b, i}
    <span
      class="cell"
      class:changed={changed(i)}
      role="listitem"
      style={`--tint:${tint(i)}`}
      aria-label={`byte ${i}${roles?.[i] ? ` (${roles[i]})` : ""}${changed(i) ? " changed" : ""}`}
    >
      <span class="val">{render(b)}</span>
      {#if roles?.[i]}<span class="role">{roles[i]}</span>{/if}
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
    flex-direction: column;
    align-items: center;
    gap: 2px;
    min-width: 2.8ch;
    padding: 0.45rem 0.4rem 0.35rem;
    background: var(--surface-alt);
    border: 1px solid var(--border);
    border-radius: 6px;
    border-top: 3px solid var(--tint);
  }
  .binary .cell {
    min-width: 9ch;
  }
  .cell .val {
    font-size: 0.98rem;
    color: var(--ink);
    letter-spacing: 0.02em;
  }
  .cell .role {
    font-size: 0.55rem;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    color: var(--ink-3);
  }
  .cell.changed {
    box-shadow: 0 0 0 2px var(--brand);
    border-color: var(--brand);
    animation: pop 0.25s ease-out;
  }
  .cell.changed .val {
    font-weight: 700;
    color: var(--brand);
  }
  @keyframes pop {
    from {
      transform: scale(0.92);
      box-shadow: 0 0 0 4px rgba(40, 87, 214, 0.3);
    }
    to {
      transform: scale(1);
      box-shadow: 0 0 0 2px var(--brand);
    }
  }
  @media (prefers-reduced-motion: reduce) {
    .cell.changed {
      animation: none;
    }
  }
</style>
