<script lang="ts">
  import { onMount } from 'svelte'
  import { invoke } from '@tauri-apps/api/core'
  import BlockLeading from '../BlockLeading.svelte'
  import BlockContent from '../BlockContent.svelte'
  import { type ParsedMarkdown } from './types'
  import { maxNestingLevel, visible } from './scripts'
  import RawContent from '../RawContent.svelte'

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

  const rawContentTextOnchange = (value: string) => {
    parseMarkdownText(value)
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

  let content: string = $state('')
  let parsedMarkdowns: ParsedMarkdown[] = $state([])
  let _maxNestingLevel = $derived.by(() => maxNestingLevel(parsedMarkdowns))

  let visibleLevel: number | null = $state(null)

  const onchange = (value: string, index: number, isHeading: boolean) => {
    if (isHeading && parsedMarkdowns[index].heading_text === value) return

    // todo: update parsedMarkdowns
  }
</script>

<main class="container">
  <input type="number" min="0" max={_maxNestingLevel} bind:value={visibleLevel} />
  <div class="d-flex row">
    <div class="col">
      <RawContent {content} textOnchange={rawContentTextOnchange} />
    </div>
    <div class="col">
      {#each parsedMarkdowns as block, i}
        <div class={`nested nest-${block.nesting_level}`}>
          {#if visible(block, visibleLevel)}
            {#if block.heading_level && 0 < block.heading_level}
              <BlockLeading
                nesting_level={block.nesting_level}
                heading_level={block.heading_level}
                heading_text={block.heading_text ?? ''}
                {visibleLevel}
                textOnchange={(value: string) => {
                  onchange(value, i, true)
                }}
                visibleLevelOnChange={(value: number) => {
                  if (visibleLevel === value) {
                    visibleLevel = null
                  } else {
                    visibleLevel = value
                  }
                }}
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
  @import './styles.css';
</style>
