<script lang="ts">
  import { onMount } from 'svelte'
  import { invoke } from '@tauri-apps/api/core'
  import BlockLeading from './Content/BlockLeading.svelte'
  import BlockContent from './Content/BlockContent.svelte'
  import RawContent from './Content/RawContent.svelte'
  import FileHandler from './Helpers/FileHandler.svelte'
  import { type ParsedMarkdown } from './types'
  import { getMaxHeadingLevel, isBlockLeadingVisible, isBlockContentVisible } from './scripts'
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
        console.log(ret) // todo
        parsedMarkdowns = ret as ParsedMarkdown[]
        if (!visibleLevel) visibleLevel = maxVisibleLevel
      })
      .catch((error: unknown) => {
        console.error(error)
        return
      })
  }

  let content: string = $state('')
  let parsedMarkdowns: ParsedMarkdown[] = $state([])
  let maxHeadingLevel = $derived.by(() => getMaxHeadingLevel(parsedMarkdowns))
  let maxVisibleLevel = $derived(maxHeadingLevel + 1)

  let visibleLevel: number | null = $state(null)

  const blockTextOnchange = (value: string, index: number, isHeading: boolean) => {
    if (isHeading && parsedMarkdowns[index].text === value) return

    parsedMarkdowns[index].text = value
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
    <input type="number" min={1} max={maxVisibleLevel} bind:value={visibleLevel} />
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
          <div class={`nested nest-${block.heading_level}`}>
            {#if block.is_heading}
              {#if isBlockLeadingVisible(block.heading_level, visibleLevel)}
                <BlockLeading
                  is_heading={block.is_heading}
                  heading_level={block.heading_level!}
                  text={block.text}
                  {visibleLevel}
                  textOnchange={(value: string) => {
                    blockTextOnchange(value, i, true)
                  }}
                  visibleLevelOnChange={(value: number) => {
                    if (visibleLevel === value) {
                      visibleLevel = maxVisibleLevel
                    } else {
                      visibleLevel = value
                    }
                  }}
                />
              {/if}
            {:else if isBlockContentVisible(block.heading_level, visibleLevel, block.text)}
              <BlockContent
                text={block.text}
                textOnchange={(value: string) => blockTextOnchange(value, i, false)}
              />
            {/if}
          </div>
        {/each}
      </div>
    {/if}
  </div>
</main>
