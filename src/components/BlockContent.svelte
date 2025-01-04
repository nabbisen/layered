<script lang="ts">
  import { onDestroy } from 'svelte'

  import { Editor } from '@tiptap/core'
  import StarterKit from '@tiptap/starter-kit'

  import BlockContentToolBar from './BlockContentToolBar.svelte'
  import { updateContent } from '../stores/editor'

  let {
    content,
    indices,
  }: {
    content: string
    indices: number[]
  } = $props()

  let editor: Editor | undefined = $state(undefined)
  let contentNode: HTMLElement | undefined = $state(undefined)
  $effect(() => {
    if (!contentNode) return
    if (editor) {
      editor.commands.setContent(content)
      return
    }
    editor = new Editor({
      element: contentNode,
      extensions: [StarterKit],
      content,
      onTransaction: () => {
        // force re-render so `editor.isActive` works as expected
        editor = editor
      },
      onUpdate() {
        updateContent(indices, editor!.getText())
      },
    })
  })
  onDestroy(() => {
    if (editor) {
      editor.destroy()
    }
  })
</script>

<BlockContentToolBar {editor} />
<div class="content" contenteditable="true" bind:this={contentNode}></div>
