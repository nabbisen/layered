<script lang="ts">
  import { onMount, onDestroy } from 'svelte'
  import { Editor } from '@tiptap/core'
  import Document from '@tiptap/extension-document'
  import Paragraph from '@tiptap/extension-paragraph'
  import Text from '@tiptap/extension-text'
  import HardBreak from '@tiptap/extension-hard-break'
  import '../../../styles/editors.css'

  const {
    markdownText,
    markdownTextOnchange,
  }: { markdownText: string; markdownTextOnchange: (updated: string) => void } = $props()

  $effect(() => {
    if (!editor) return
    editor.commands.setContent(markdownText, false, { preserveWhitespace: 'full' })
    return
  })

  let element: HTMLDivElement

  let editor: Editor | undefined = $state()
  onMount(() => {
    editor = new Editor({
      element: element,
      extensions: [Document, Paragraph, Text, HardBreak],
      content: '',
      onTransaction: () => {
        // force re-render so `editor.isActive` works as expected
        editor = editor
      },
      onBlur: () => {
        if (!editor) return
        markdownTextOnchange(editor.getText())
      },
    })
  })

  onDestroy(() => {
    if (editor) editor.destroy()
  })
</script>

<div bind:this={element}></div>
