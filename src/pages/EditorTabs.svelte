<script lang="ts">
  import { onMount } from 'svelte'
  import EditorTab from '../components/editors/EditorTab.svelte'
  import { invoke } from '@tauri-apps/api/core'

  let markdownTexts: string[] = $state([])

  onMount(async () => {
    // todo dev dummy
    const markdownText = (await invoke('ready', {})) as string
    markdownTexts = [markdownText]
  })
</script>

{#each markdownTexts as markdownText}
  <EditorTab {markdownText} />
{/each}
