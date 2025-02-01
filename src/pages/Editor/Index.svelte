<script lang="ts">
  import { onMount } from 'svelte'
  import { invoke } from '@tauri-apps/api/core'
  import BlockLeading from './Content/BlockLeading.svelte'
  import BlockContent from './Content/BlockContent.svelte'
  import RawContent from './Content/RawContent.svelte'
  import FileHandler from './Helpers/FileHandler.svelte'
  import { type ParsedMarkdown } from './types'
  import { maxNestingLevel, visible } from './scripts'
  import './styles.css'

  onMount(() => {
    // todo dev dummy
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

  const rawTextOnchange = (value: string) => {
    content = value
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

  const blockTextOnchange = (value: string, index: number, isHeading: boolean) => {
    if (isHeading && parsedMarkdowns[index].heading_text === value) return

    if (isHeading) {
      parsedMarkdowns[index].heading_text = value
    } else {
      parsedMarkdowns[index].html = value
    }
    invoke('compose', { parsedMarkdowns: parsedMarkdowns })
      .then((ret: unknown) => {
        content = ret as string
      })
      .catch((error: unknown) => {
        console.error(error)
        return
      })
  }

  type EditorLayout = 'raw' | 'both' | 'layers'
  const EDITOR_LAYOUTS: EditorLayout[] = ['raw', 'both', 'layers']
  let activeEditor: EditorLayout = $state('layers')

  const isRawEditorVisible = (): boolean => {
    const matchers: EditorLayout[] = ['raw', 'both']
    return matchers.includes(activeEditor)
  }
  const isLayersEditorVisible = (): boolean => {
    const matchers: EditorLayout[] = ['layers', 'both']
    return matchers.includes(activeEditor)
  }
</script>

<FileHandler
  {parsedMarkdowns}
  rawContentOnChange={(rawContent: string) => {
    content = rawContent
    parseMarkdownText(content)
  }}
/>
<main class="container editor">
  <nav>
    <input type="number" min="0" max={_maxNestingLevel} bind:value={visibleLevel} />
    <div class="d-flex">
      {#each EDITOR_LAYOUTS as editorLayout}
        <label
          ><input
            type="radio"
            name="active-editor"
            bind:group={activeEditor}
            value={editorLayout}
          />{editorLayout}</label
        >
      {/each}
    </div>
  </nav>
  <div class="row">
    {#if isRawEditorVisible()}
      <div class="col">
        <RawContent {content} textOnchange={rawTextOnchange} />
      </div>
    {/if}
    {#if isLayersEditorVisible()}
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
                    blockTextOnchange(value, i, true)
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
                  textOnchange={(value: string) => blockTextOnchange(value, i, false)}
                />
              {/if}
            {/if}
          </div>
        {/each}
      </div>
    {/if}
  </div>
</main>
