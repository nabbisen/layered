<script lang="ts">
  import { onMount } from 'svelte'
  import { invoke } from '@tauri-apps/api/core'

  import { EDITOR_LAYOUTS } from '../../constants'
  import { type EditorLayout, type ParsedMarkdown } from '../../types'
  import FileHandler from './FileHandler.svelte'
  import TextEditor from './TextEditor/TextEditor.svelte'
  import Editor from './TreeEditor/TreeEditor.svelte'

  const DEFAULT_EDITOR_LAYOUT: EditorLayout = 'layers'

  let content: string = $state('')
  let parsedMarkdowns: ParsedMarkdown[] = $state([])

  let activeEditor: EditorLayout = $state(DEFAULT_EDITOR_LAYOUT)

  onMount(async () => {
    // todo dev dummy
    const markdownText = (await invoke('ready', {})) as string
    await updateEditorContent(markdownText)
  })

  const parseMarkdownText = async (markdownText: string): Promise<ParsedMarkdown[]> => {
    return (await invoke('parse', { markdownText: markdownText })) as ParsedMarkdown[]
  }

  const updateEditorContent = async (markdownText: string) => {
    parsedMarkdowns = await parseMarkdownText(markdownText)
    console.log($state.snapshot(parsedMarkdowns)) // todo

    content = markdownText
  }

  const textEditorContentOnchange = (updated: string) => {
    content = updated
    parseMarkdownText(updated)
  }

  const isRawEditorVisibleMatchers: EditorLayout[] = ['raw', 'both']
  const isRawEditorVisible: boolean = $derived(isRawEditorVisibleMatchers.includes(activeEditor))

  const isLayersEditorVisibleMatchers: EditorLayout[] = ['layers', 'both']
  const isLayersEditorVisible: boolean = $derived(
    isLayersEditorVisibleMatchers.includes(activeEditor)
  )
</script>

<nav class="d-flex">
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

<div class="editor">
  {#if isLayersEditorVisible}
    {#key parsedMarkdowns}
      <Editor
        {parsedMarkdowns}
        parsedMarkdownsOnChange={(updated: ParsedMarkdown[]) => (parsedMarkdowns = updated)}
        contentOnChange={(updated: string) => (content = updated)}
      />
    {/key}
  {/if}
  {#if isRawEditorVisible}
    <div class="col">
      <TextEditor {content} textOnchange={textEditorContentOnchange} />
    </div>
  {/if}

  <FileHandler
    {parsedMarkdowns}
    markdownTextOnChange={(markdownText: string) => {
      updateEditorContent(markdownText)
    }}
  />
</div>
