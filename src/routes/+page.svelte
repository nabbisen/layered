<script lang="ts">
  import Editor from '../components/Editor.svelte'
  import type { NestedStringArray } from '../type'

  const src: NestedStringArray = [
    'aaa',
    'aaaaa',
    ['aab', 'aaaab', ['aac', 'aaaac', 'aad', ['aaaad']]],
    'aa',
  ]
</script>

<main class="container">
  <Editor {src} />
</main>

<style>
</style>
