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

  type ActiveEditor = 'raw' | 'both' | 'layers'
  let activeEditor: ActiveEditor = $state('layers')
</script>

<main class="container editor">
  <nav>
    <input type="number" min="0" max={_maxNestingLevel} bind:value={visibleLevel} />
    <div class="d-flex">
      <label
        ><input type="radio" name="active-editor" bind:group={activeEditor} value="raw" />raw</label
      >
      <label
        ><input
          type="radio"
          name="active-editor"
          bind:group={activeEditor}
          value="both"
        />both</label
      >
      <label
        ><input
          type="radio"
          name="active-editor"
          bind:group={activeEditor}
          value="layers"
        />layers</label
      >
    </div>
  </nav>
  <div class="row">
    {#if ['raw', 'both'].includes(activeEditor)}
      <div class="col">
        <RawContent {content} textOnchange={rawContentTextOnchange} />
      </div>
    {/if}
    {#if ['layers', 'both'].includes(activeEditor)}
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
    {/if}
  </div>
</main>

<style>
  .nested {
    padding-left: 4.4rem;
  }

  .nested.nest-0 {
    padding-left: 0;
  }

  .nested.nest-1 {
    padding-left: 0.9rem;
  }

  .nested.nest-2 {
    padding-left: 1.8rem;
  }

  .nested.nest-3 {
    padding-left: 2.7rem;
  }

  .nested.nest-4 {
    padding-left: 3.6rem;
  }
</style>
