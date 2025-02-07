<script lang="ts">
  import { onMount } from 'svelte'
  import { invoke } from '@tauri-apps/api/core'

  import { EDITOR_LAYOUTS } from '../../constants'
  import { type EditorLayout, type ParsedMarkdown } from '../../types'
  import FileHandler from './FileHandler.svelte'
  import TextEditor from './TextEditor/TextEditor.svelte'
  import TreeEditor from './TreeEditor/TreeEditor.svelte'
  import LayerEditor from './LayerEditor/LayerEditor.svelte'

  const DEFAULT_EDITOR_LAYOUT: EditorLayout = 'layer'

  const { markdownText }: { markdownText: string } = $props()

  let _markdownText: string = $state(markdownText)
  let parsedMarkdowns: ParsedMarkdown[] = $state([])

  let activeEditor: EditorLayout = $state(DEFAULT_EDITOR_LAYOUT)

  onMount(async () => {
    await updateEditorContent(_markdownText)
  })

  const parseMarkdownText = async (markdownText: string): Promise<ParsedMarkdown[]> => {
    return (await invoke('parse', { markdownText: markdownText })) as ParsedMarkdown[]
  }

  const updateEditorContent = async (markdownText: string) => {
    parsedMarkdowns = await parseMarkdownText(markdownText)
    console.log($state.snapshot(parsedMarkdowns)) // todo

    _markdownText = markdownText
  }

  const textEditorContentOnchange = (updated: string) => {
    _markdownText = updated
    parseMarkdownText(updated)
  }
</script>

<div class="editor-tab">
  <div class="editor">
    {#if activeEditor === 'layer'}
      <LayerEditor
        {parsedMarkdowns}
        parsedMarkdownsOnChange={(updated) => {
          parsedMarkdowns = updated
          // todo: update md text
        }}
      />
    {:else if activeEditor === 'tree'}
      {#key parsedMarkdowns}
        <TreeEditor
          {parsedMarkdowns}
          parsedMarkdownsOnChange={(updated: ParsedMarkdown[]) => (parsedMarkdowns = updated)}
          contentOnChange={(updated: string) => (_markdownText = updated)}
        />
      {/key}
    {:else if activeEditor === 'text'}
      <div class="col">
        <TextEditor markdownText={_markdownText} markdownTextOnchange={textEditorContentOnchange} />
      </div>
    {/if}
  </div>

  <aside>
    <nav class="d-flex">
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
    </nav>
  </aside>

  <footer>
    <nav>
      <FileHandler
        {parsedMarkdowns}
        markdownTextOnChange={(markdownText: string) => {
          updateEditorContent(markdownText)
        }}
      />
    </nav>
  </footer>
</div>
