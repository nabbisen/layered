<script lang="ts">
  import { onMount, onDestroy } from 'svelte'
  import { getCurrentWebview, type DragDropEvent } from '@tauri-apps/api/webview'
  import { type UnlistenFn, type Event as TauriEvent } from '@tauri-apps/api/event'

  const { onDrop }: { onDrop: (filepath: string) => void } = $props()

  let filepath: string | undefined
  let unlistenDragDrop: UnlistenFn | undefined

  onMount(listenDragDrop)

  onDestroy(() => {
    if (unlistenDragDrop) unlistenDragDrop()
  })

  async function listenDragDrop() {
    unlistenDragDrop = await getCurrentWebview().onDragDropEvent(
      (event: TauriEvent<DragDropEvent>) => {
        if (event.payload.type === 'drop') {
          handleDrop(event.payload.paths)
        }
      }
    )
  }

  function handleDrop(paths: string[]) {
    // todo: single file only now
    filepath = paths[0]
    onDrop(filepath)
  }
</script>
