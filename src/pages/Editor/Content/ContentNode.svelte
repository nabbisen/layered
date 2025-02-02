<script lang="ts">
  import { onMount, onDestroy } from 'svelte'
  import { Editor } from '@tiptap/core'
  import StarterKit from '@tiptap/starter-kit'

  const { text, textOnchange }: { text: string; textOnchange: Function } = $props()

  let element: HTMLDivElement

  let editor: Editor | undefined = $state()
  onMount(() => {
    editor = new Editor({
      element: element,
      extensions: [StarterKit],
      content: text,
      onTransaction: () => {
        // force re-render so `editor.isActive` works as expected
        editor = editor
      },
      onBlur: () => {
        if (!editor) return
        textOnchange(editor.getHTML())
      },
    })
  })

  onDestroy(() => {
    if (editor) editor.destroy()
  })
</script>

<div bind:this={element}></div>
