<script lang="ts">
  import { onMount } from 'svelte'
  import { Editor } from '@tiptap/core'
  import StarterKit from '@tiptap/starter-kit'

  const { content, onchange }: { content: string; onchange: Function } = $props()

  $effect(() => {
    if (!editor) return
    editor.commands.setContent(content.replaceAll('\n', '<br>'))
    return
  })

  let element: HTMLDivElement

  let editor: Editor | undefined = $state()
  onMount(() => {
    editor = new Editor({
      element: element,
      extensions: [StarterKit],
      content: '',
      onTransaction: () => {
        // force re-render so `editor.isActive` works as expected
        editor = editor
      },
      onBlur: () => {
        if (!editor) return
        onchange(editor.getText())
      },
    })
  })
</script>

<div bind:this={element}></div>
