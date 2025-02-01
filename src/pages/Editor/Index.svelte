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
  let visibleLevel: number | null = $state(null)

  let maxHeadingLevel = $derived.by(() => getMaxHeadingLevel(parsedMarkdowns))
  let maxVisibleLevel = $derived(maxHeadingLevel + 1)

  const addBlockNode = (
    index: number,
    isHeading: boolean,
    headinLevel: number,
    parentNodeId: number | null,
    ancestors: number[]
  ) => {
    parsedMarkdowns.splice(index, 0, {
      nodeId: parsedMarkdowns.length + 1,
      isHeading: isHeading,
      headingLevel: headinLevel,
      text: '',
      parentNodeId: parentNodeId,
      ancestors: ancestors,
    } as ParsedMarkdown)
    parsedMarkdowns = [...parsedMarkdowns]
  }

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

<main class="container editor">
  <nav class="d-flex">
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
          <div class="line" data-line={i + 1}>
            <div class={`nested nest-${block.headingLevel}`}>
              {#if block.isHeading}
                {#if isBlockLeadingVisible(block.headingLevel, visibleLevel)}
                  <BlockLeading
                    isHeading={block.isHeading}
                    headingLevel={block.headingLevel!}
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
                    childrenVisibleOnChange={() => {
                      // todo
                      console.log(123)
                    }}
                    addSiblingHeading={() =>
                      addBlockNode(
                        i + 1,
                        true,
                        block.headingLevel,
                        block.parentNodeId,
                        block.ancestors
                      )}
                    addChildHeading={() =>
                      addBlockNode(
                        i + 1,
                        true,
                        block.headingLevel + 1,
                        block.parentNodeId,
                        block.ancestors
                      )}
                    addChildContent={() =>
                      addBlockNode(i + 1, false, block.headingLevel, block.nodeId, [
                        ...block.ancestors,
                        block.nodeId,
                      ])}
                  />
                {/if}
              {:else if isBlockContentVisible(block.headingLevel, visibleLevel, block.text)}
                <BlockContent
                  text={block.text}
                  textOnchange={(value: string) => blockTextOnchange(value, i, false)}
                />
              {/if}
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</main>
<FileHandler
  {parsedMarkdowns}
  rawContentOnChange={(rawContent: string) => {
    content = rawContent
    parseMarkdownText(content)
  }}
/>
