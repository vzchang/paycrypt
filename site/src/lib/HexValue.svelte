<script lang="ts">
  export let value: string;
  let copied = false;
  let timer: ReturnType<typeof setTimeout>;

  async function copy() {
    try {
      await navigator.clipboard.writeText(value);
      copied = true;
      clearTimeout(timer);
      timer = setTimeout(() => (copied = false), 1200);
    } catch {
      /* clipboard unavailable; ignore */
    }
  }
</script>

<button class="hexval" on:click={copy} title="click to copy" aria-label={`copy ${value}`}>
  <span class="text">{value}</span>
  <span class="tag" class:copied>{copied ? "copied" : "copy"}</span>
</button>

<style>
  .hexval {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    font-family: var(--mono);
    font-size: 0.88rem;
    letter-spacing: 0.04em;
    color: var(--text-secondary);
    background: none;
    border: none;
    padding: 0.15rem 0;
    cursor: pointer;
    text-align: left;
    word-break: break-all;
  }
  .text {
    border-bottom: 1px dashed transparent;
  }
  .hexval:hover .text {
    border-bottom-color: var(--border-strong);
    color: var(--text-primary);
  }
  .tag {
    flex: 0 0 auto;
    font-size: 0.6rem;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--text-muted);
    border: 1px solid var(--border);
    border-radius: 999px;
    padding: 0.05rem 0.45rem;
    opacity: 0;
    transition: opacity 0.15s;
  }
  .hexval:hover .tag,
  .tag.copied {
    opacity: 1;
  }
  .tag.copied {
    color: var(--role-pin);
    border-color: var(--role-pin);
  }
</style>
