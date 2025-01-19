<script lang="ts">
  import Block from './Block.svelte'
  import BlockContent from './BlockContent.svelte'
  import { dragStart, drop } from '../stores/blockDragDrop'
  import type { NestedStringArray } from '../type'

  const editorConfig = {
    namespace: 'MyEditor',
    theme: {},
    onError: console.error,
  }

  let {
    indices,
    content,
  }: {
    indices: number[]
    content: NestedStringArray
  } = $props()

  function handleDragStart() {
    dragStart(indices)
  }
  function handleDrop() {
    drop(indices)
  }
</script>

<div class={`level-${indices.length}`}>
  {#if Array.isArray(content)}
    {#each content as child, i}
      <Block indices={[...indices, i]} content={child} />
    {/each}
  {:else}
    <div onpointerdown={handleDragStart} onpointerup={handleDrop}>
      <span class="op">---</span>
      <BlockContent {content} {indices} />
    </div>
  {/if}
</div>

<style>
  .op {
    cursor: grab;
  }
  .content {
    padding: 0 1.1rem;
    margin-left: 0.8rem;
  }
  .level-1 {
    margin-left: 0.5rem;
  }
  .level-2 {
    margin-left: 1rem;
  }
  .level-3 {
    margin-left: 1.5rem;
  }
  .level-4 {
    margin-left: 2rem;
  }
  .level-5 {
    margin-left: 2.5rem;
  }
  .level-6 {
    margin-left: 3rem;
  }
</style>
