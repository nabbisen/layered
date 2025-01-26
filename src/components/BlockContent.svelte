<script lang="ts">
  import { onMount } from 'svelte'
  import { Editor } from '@tiptap/core'
  import StarterKit from '@tiptap/starter-kit'

  const { html, onchange }: { html: string; onchange: Function } = $props()

  let element: HTMLDivElement

  let editor: Editor | undefined = $state()
  onMount(() => {
    editor = new Editor({
      element: element,
      extensions: [StarterKit],
      content: html,
      onTransaction: () => {
        // force re-render so `editor.isActive` works as expected
        editor = editor
      },
      onUpdate: () => {
        if (!editor) return
        onchange(editor.getText())
      },
    })
  })
</script>

<div bind:this={element}></div>
