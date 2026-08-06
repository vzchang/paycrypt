<script lang="ts">
  import { onMount } from "svelte";
  export let active = false;

  let canvas: HTMLCanvasElement;

  onMount(() => {
    const ctx = canvas.getContext("2d")!;
    let raf = 0;
    let cols: number[] = [];
    const glyphs = "0123456789ABCDEF".split("");
    const reduce = window.matchMedia("(prefers-reduced-motion: reduce)").matches;

    function resize() {
      canvas.width = window.innerWidth;
      canvas.height = window.innerHeight;
      cols = Array(Math.ceil(canvas.width / 14)).fill(0);
    }
    resize();
    window.addEventListener("resize", resize);

    function frame() {
      ctx.fillStyle = "rgba(0,6,0,0.08)";
      ctx.fillRect(0, 0, canvas.width, canvas.height);
      ctx.fillStyle = "#33cc33";
      ctx.font = "13px monospace";
      for (let i = 0; i < cols.length; i++) {
        const ch = glyphs[Math.floor(Math.random() * glyphs.length)];
        ctx.fillText(ch, i * 14, cols[i] * 14);
        cols[i] = cols[i] * 14 > canvas.height && Math.random() > 0.975 ? 0 : cols[i] + 1;
      }
      raf = requestAnimationFrame(frame);
    }
    if (!reduce) raf = requestAnimationFrame(frame);

    return () => {
      cancelAnimationFrame(raf);
      window.removeEventListener("resize", resize);
    };
  });
</script>

<canvas bind:this={canvas} class:active aria-hidden="true"></canvas>

<style>
  canvas {
    position: fixed;
    inset: 0;
    z-index: 0;
    opacity: 0;
    pointer-events: none;
    transition: opacity 0.4s;
  }
  canvas.active {
    opacity: 0.25;
  }
</style>
