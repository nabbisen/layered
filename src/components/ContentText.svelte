<script lang="ts">
  const {
    text,
    layerLevel,
    hasChildren,
    oninput,
  }: { text: string; layerLevel: number; hasChildren: boolean | undefined; oninput: Function } =
    $props()

  const _oninput = (e: Event & { currentTarget: EventTarget & HTMLElement }) => {
    oninput(e.currentTarget.innerText)
  }
</script>

{#if hasChildren}
  <div role="heading" aria-level={layerLevel} contenteditable oninput={_oninput}>
    {text}
  </div>
{:else}
  <div contenteditable oninput={_oninput}>{text}</div>
{/if}
