<script lang="ts">
  import { onMount } from 'svelte'

  import { initContent, subscribeContent } from '../stores/editor'
  import Block from './Block.svelte'
  import type { NestedStringArray } from '../type'

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
  $effect(() => {
    subscribeContent((value: NestedStringArray) => (content = value))
  })
</script>

{#if content && 0 < content.length}
  <Block {indices} {content} />
{/if}
