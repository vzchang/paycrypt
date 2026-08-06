<script lang="ts">
  // Only the two stages this site implements are highlighted.
  const stages = [
    { label: "PIN entered", impl: false },
    { label: "PIN block", impl: true },
    { label: "DUKPT key", impl: true },
    { label: "encrypt", impl: true },
    { label: "acquirer", impl: false },
    { label: "network", impl: false },
    { label: "issuer verify", impl: false },
  ];
</script>

<nav class="flow" aria-label="card-present transaction pipeline">
  {#each stages as s, i}
    <span class="stage" class:impl={s.impl}>{s.label}</span>
    {#if i < stages.length - 1}<span class="arrow" aria-hidden="true">→</span>{/if}
  {/each}
</nav>
<p class="flow-key">
  <span class="chip impl"></span> built by paycrypt &nbsp;·&nbsp;
  <span class="chip"></span> narrated context
</p>

<style>
  .flow {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 0.4rem;
    margin: 1.4rem 0 0.4rem;
    padding: 0.95rem 1.1rem;
    background: var(--surface-1);
    border: 1px solid var(--border);
    border-radius: var(--radius);
  }
  .stage {
    font-family: var(--mono);
    font-size: 0.74rem;
    padding: 0.32rem 0.62rem;
    border-radius: 6px;
    background: var(--surface-2);
    color: var(--text-muted);
    white-space: nowrap;
  }
  .stage.impl {
    background: var(--accent-dim);
    color: var(--accent-bright);
    border: 1px solid rgba(212, 175, 106, 0.4);
  }
  .arrow {
    color: var(--border-strong);
    font-size: 0.8rem;
  }
  .flow-key {
    font-size: 0.74rem;
    color: var(--text-muted);
    margin: 0.5rem 0 0;
  }
  .chip {
    display: inline-block;
    width: 0.65rem;
    height: 0.65rem;
    border-radius: 3px;
    background: var(--surface-2);
    vertical-align: -1px;
  }
  .chip.impl {
    background: var(--accent);
    border: 1px solid var(--accent-bright);
  }
</style>
