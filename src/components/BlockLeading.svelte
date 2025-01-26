<script lang="ts">
  const {
    heading_level,
    heading_text,
    onchange,
  }: { heading_level: number; heading_text: string; onchange: Function } = $props()
</script>

{#if heading_level <= 6}
  <svelte:element
    this={`h${heading_level}`}
    onblur={(e: FocusEvent & { currentTarget: EventTarget & HTMLElement }) =>
      onchange(e.currentTarget.innerText)}
    contenteditable
  >
    {heading_text}
  </svelte:element>
{:else}
  <div
    class={`h${heading_level}`}
    onblur={(e: FocusEvent & { currentTarget: EventTarget & HTMLElement }) =>
      onchange(e.currentTarget.innerText)}
    contenteditable
  >
    {heading_text}
  </div>
{/if}

<style>
  h1,
  h2,
  h3,
  h4,
  h5,
  h6 {
    padding: 0;
    margin: 0;
  }
</style>
