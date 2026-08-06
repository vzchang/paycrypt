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
    gap: 0.55rem;
    font-family: var(--mono);
    font-size: 0.9rem;
    letter-spacing: 0.03em;
    color: var(--ink-2);
    background: none;
    border: none;
    padding: 0.2rem 0;
    min-height: auto;
    cursor: pointer;
    text-align: left;
    word-break: break-all;
    font-weight: 400;
  }
  .text {
    border-bottom: 1px dashed transparent;
  }
  .hexval:hover .text {
    border-bottom-color: var(--border-strong);
    color: var(--ink);
  }
  .tag {
    flex: 0 0 auto;
    font-size: 0.62rem;
    font-weight: 600;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    color: var(--ink-3);
    border: 1px solid var(--border-strong);
    border-radius: 999px;
    padding: 0.1rem 0.5rem;
    opacity: 0;
    transition: opacity 0.15s;
  }
  .hexval:hover .tag,
  .tag.copied {
    opacity: 1;
  }
  .tag.copied {
    color: var(--ok);
    border-color: var(--ok);
  }
</style>
