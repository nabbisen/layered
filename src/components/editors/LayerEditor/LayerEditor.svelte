<script lang="ts">
  import { parse } from 'svelte/compiler'
  import type { ParsedMarkdown } from '../../../types'

  const {
    parsedMarkdowns,
    parsedMarkdownsOnChange,
  }: {
    parsedMarkdowns: ParsedMarkdown[]
    parsedMarkdownsOnChange: (parsedMarkdowns: ParsedMarkdown[]) => void
  } = $props()

  let parentNodeId: number | null = $state(1)

  const filteredParsedMarkdowns: ParsedMarkdown[] = $derived(
    parsedMarkdowns.filter((x) => x.parentNodeId === parentNodeId)
  )
</script>

{#each filteredParsedMarkdowns as parsedMarkdown}
  <div class="d-flex">
    {#if parentNodeId !== null}
      <button
        onclick={() =>
          (parentNodeId = parsedMarkdowns.find((x) => x.nodeId === parentNodeId)!.parentNodeId)}
        >-|</button
      >
    {/if}
    <!-- todo: onblur w/ text editor -->
    <div contenteditable>{parsedMarkdown.text}</div>
    <button
      onclick={() => {
        const ret = [...parsedMarkdowns]
        const index = parsedMarkdowns.findIndex((x) => x.nodeId === parsedMarkdown.nodeId)
        // todo: integrate and place fn to use Array.splice() in scripts.ts
        ret.splice(index + 1, 0, {
          nodeId: parsedMarkdowns.length + 1,
          isHeading: true,
          headingLevel: parsedMarkdown.headingLevel,
          text: '',
          parentNodeId: parsedMarkdown.parentNodeId,
          ancestors: parsedMarkdown.ancestors,
          visible: true,
        } as ParsedMarkdown)
        parsedMarkdownsOnChange(ret)
      }}>|=</button
    >
    {#if parsedMarkdowns.some((x) => x.parentNodeId === parsedMarkdown.nodeId)}
      <button onclick={() => (parentNodeId = parsedMarkdown.nodeId)}>|_</button>
    {/if}
  </div>
{/each}
