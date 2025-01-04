<script lang="ts">
  import { onMount } from 'svelte'

  import { getIndicedsContent, initContent, subscribeContent } from '../stores/editor'
  import Block from './Block.svelte'
  import type { NestedStringArray } from '../type'
  import {
    subscribeFromDragDropItem,
    dragCancel,
    type dragDropItemType,
  } from '../stores/blockDragDrop'

  let {
    src,
  }: {
    src: NestedStringArray
  } = $props()

  const indices: number[] = []

  onMount(() => {
    initContent(src)
  })

  let content: NestedStringArray | undefined = $state(undefined)
  let dragged: string | undefined = $state(undefined)
  $effect(() => {
    subscribeContent((value: NestedStringArray) => (content = value))
    subscribeFromDragDropItem(
      (value: dragDropItemType | undefined) =>
        (dragged = value ? getIndicedsContent(value.indices) : undefined)
    )
  })
</script>

{#if content && 0 < content.length}
  <div
    onpointerleave={dragCancel}
    onpointerup={dragCancel}
    onpointermove={(event) => {
      if (dragged) event.preventDefault()
    }}
  >
    <Block {indices} {content} />
    {#if dragged}
      <div
        style="position: fixed; top: 0; right: 0; width: 100px; height: 100px; background-color: orange;"
      >
        Dragged:<br />
        {dragged}
      </div>
    {/if}
  </div>
{/if}
