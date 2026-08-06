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
    padding: 1rem 1.1rem;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    box-shadow: var(--shadow);
  }
  .stage {
    font-family: var(--mono);
    font-size: 0.78rem;
    padding: 0.35rem 0.65rem;
    border-radius: 6px;
    background: var(--surface-alt);
    color: var(--ink-3);
    white-space: nowrap;
  }
  .stage.impl {
    background: var(--brand-weak);
    color: var(--brand);
    border: 1px solid rgba(40, 87, 214, 0.3);
    font-weight: 600;
  }
  .arrow {
    color: var(--border-strong);
    font-size: 0.85rem;
  }
  .flow-key {
    font-size: 0.78rem;
    color: var(--ink-3);
    margin: 0.5rem 0 0;
  }
  .chip {
    display: inline-block;
    width: 0.7rem;
    height: 0.7rem;
    border-radius: 3px;
    background: var(--surface-alt);
    border: 1px solid var(--border-strong);
    vertical-align: -1px;
  }
  .chip.impl {
    background: var(--brand);
    border-color: var(--brand);
  }
</style>
