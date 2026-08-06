<script lang="ts">
  const themes = [
    { id: "", label: "Console" },
    { id: "neon", label: "Neon" },
    { id: "green", label: "CRT green" },
    { id: "amber", label: "Amber" },
    { id: "matrix", label: "Matrix" },
  ];
  export let active = "";

  function pick(id: string) {
    active = id;
    if (id) document.documentElement.setAttribute("data-term", id);
    else document.documentElement.removeAttribute("data-term");
  }
</script>

<div class="picker" role="group" aria-label="theme">
  <span class="lbl">theme</span>
  {#each themes as t}
    <button class:on={active === t.id} on:click={() => pick(t.id)}>{t.label}</button>
  {/each}
</div>

<style>
  .picker {
    position: sticky;
    top: 0;
    z-index: 10;
    display: flex;
    align-items: center;
    gap: 0.4rem;
    flex-wrap: wrap;
    padding: 0.7rem 0;
    margin-bottom: 0.5rem;
    background: color-mix(in srgb, var(--surface-0) 88%, transparent);
    backdrop-filter: blur(6px);
  }
  .lbl {
    font-family: var(--mono);
    font-size: 0.7rem;
    letter-spacing: 0.12em;
    text-transform: uppercase;
    color: var(--text-muted);
    margin-right: 0.3rem;
  }
  button {
    font-family: var(--mono);
    font-size: 0.78rem;
    color: var(--text-secondary);
    background: var(--surface-2);
    border: 1px solid var(--border-strong);
    border-radius: 6px;
    padding: 0.3rem 0.7rem;
    cursor: pointer;
  }
  button:hover {
    border-color: var(--accent);
  }
  button.on {
    color: var(--surface-0);
    background: var(--accent);
    border-color: var(--accent);
  }
</style>
