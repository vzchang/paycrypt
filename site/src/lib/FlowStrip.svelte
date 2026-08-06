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
    padding: 0.9rem 1rem;
    background: var(--surface-1);
    border: 1px solid var(--border);
    border-radius: var(--radius);
  }
  .stage {
    font-family: var(--mono);
    font-size: 0.74rem;
    padding: 0.3rem 0.6rem;
    border-radius: 5px;
    background: var(--surface-2);
    color: var(--text-muted);
    white-space: nowrap;
  }
  .stage.impl {
    background: rgba(57, 135, 229, 0.14);
    color: #86b6ef;
    border: 1px solid rgba(57, 135, 229, 0.35);
  }
  .arrow {
    color: var(--border-strong);
    font-size: 0.8rem;
  }
  .flow-key {
    font-size: 0.72rem;
    color: var(--text-muted);
    margin: 0.4rem 0 0;
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
    background: rgba(57, 135, 229, 0.5);
    border: 1px solid rgba(57, 135, 229, 0.6);
  }
</style>
