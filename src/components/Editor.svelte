<script lang="ts">
  import { onMount } from 'svelte'
  import { invoke } from '@tauri-apps/api/core'
  import BlockLeading from './BlockLeading.svelte'
  import BlockContent from './BlockContent.svelte'
  import { type ParsedMarkdown } from './Editor'

  onMount(() => {
    invoke('ready', {})
      .then((ret: unknown) => {
        content = ret as string
        parseMarkdownText(content)
      })
      .catch((error: unknown) => {
        console.error(error)
        return
      })
  })

  const markdownTextOnchange = (
    e: Event & { currentTarget: EventTarget & HTMLTextAreaElement }
  ) => {
    parseMarkdownText(e.currentTarget.value)
  }

  const parseMarkdownText = (markdownText: string) => {
    invoke('parse', { markdownText: markdownText })
      .then((ret: unknown) => {
        console.log(ret)
        parsedMarkdowns = ret as ParsedMarkdown[]
      })
      .catch((error: unknown) => {
        console.error(error)
        return
      })
  }

  let visibleLevel: number | null = $state(null)

  const visible = (block: ParsedMarkdown): boolean => {
    if (!visibleLevel || Number.isNaN(visibleLevel) || Number(visibleLevel) <= 0) {
      return true
    }
    return block.heading_level ? block.heading_level <= Number(visibleLevel) : false
  }

  let content: string = $state('')
  let parsedMarkdowns: ParsedMarkdown[] = $state([])
  let maxNestingLevel = $derived.by(() => {
    let maxLevel: number = 0
    parsedMarkdowns.forEach((x: ParsedMarkdown) => {
      if (maxLevel < x.nesting_level) maxLevel = x.nesting_level
    })
    return maxLevel
  })

  const onchange = (value: string, index: number, isHeading: boolean) => {
    if (isHeading && parsedMarkdowns[index].heading_text === value) return

    // todo: update parsedMarkdowns
  }
</script>

<main class="container">
  <input type="number" min="0" max={maxNestingLevel} bind:value={visibleLevel} />
  <div class="d-flex row">
    <textarea class="col" onchange={markdownTextOnchange} bind:value={content}></textarea>
    <div class="col">
      {#each parsedMarkdowns as block, i}
        <div class={`nested nest-${block.nesting_level}`}>
          {#if visible(block)}
            {#if block.heading_level && 0 < block.heading_level}
              <BlockLeading
                heading_level={block.heading_level}
                heading_text={block.heading_text ?? ''}
                onchange={(value: string) => onchange(value, i, true)}
              />
            {:else}
              <BlockContent
                html={block.html ?? ''}
                onchange={(value: string) => onchange(value, i, false)}
              />
            {/if}
          {/if}
        </div>
      {/each}
    </div>
  </div>
</main>

<style>
  @import './Editor.css';
</style>
