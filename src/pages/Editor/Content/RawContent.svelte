<script lang="ts">
  import { onMount } from 'svelte'
  import { Editor } from '@tiptap/core'
  import Document from '@tiptap/extension-document'
  import Paragraph from '@tiptap/extension-paragraph'
  import Text from '@tiptap/extension-text'
  import HardBreak from '@tiptap/extension-hard-break'

  const { content, textOnchange }: { content: string; textOnchange: Function } = $props()

  $effect(() => {
    if (!editor) return
    editor.commands.setContent(content, false, { preserveWhitespace: 'full' })
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
        textOnchange(editor.getText())
      },
    })
  })
</script>

<div bind:this={element}></div>
