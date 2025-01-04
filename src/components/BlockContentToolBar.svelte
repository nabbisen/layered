<script module>
  import { Editor } from '@tiptap/core'
</script>

<script lang="ts">
  let {
    editor,
  }: {
    editor: Editor | undefined
  } = $props()

  function bold() {
    editor?.chain().focus().toggleBold().run()
  }

  function italic() {
    editor?.chain().focus().toggleItalic().run()
  }
</script>

<button onclick={bold}>B</button>
<button onclick={italic}><i>I</i></button>
