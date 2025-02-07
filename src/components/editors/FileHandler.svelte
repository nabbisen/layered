<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import { open as tauriDialogOpen, save as tauriDialogSave } from '@tauri-apps/plugin-dialog'

  import DragDrop from '../../components/common/DragDrop.svelte'
  import { type ParsedMarkdown } from '../../types'

  const {
    parsedMarkdowns,
    markdownTextOnChange,
  }: {
    parsedMarkdowns: ParsedMarkdown[]
    markdownTextOnChange: (markdownText: string) => void
  } = $props()

  let openedFilepath: string | undefined = $state()

  const openOnClick = async () => {
    const filepath: string | null = await tauriDialogOpen({
      filters: [
        {
          name: 'Markdown',
          extensions: ['md'],
        },
      ],
    })
    // todo
    if (!filepath) return
    open(filepath)
  }

  const open = (filepath: string) => {
    invoke('open', { filepath: filepath })
      .then((ret: unknown) => {
        openedFilepath = filepath
        markdownTextOnChange(ret as string)
      })
      .catch((error: unknown) => {
        console.error(error)
        return
      })
  }

  const saveToOverwrite = async () => {
    await save(openedFilepath!)
  }

  const saveAs = async () => {
    const filepath: string | null = await tauriDialogSave({
      filters: [
        {
          name: 'Markdown',
          extensions: ['md'],
        },
      ],
    })
    // todo
    if (!filepath) return
    save(filepath).then(() => (openedFilepath = filepath))
  }

  const save = async (filepath: string) => {
    invoke('save', { parsedMarkdowns: parsedMarkdowns, filepath: filepath }).catch(
      (error: unknown) => {
        console.error(error)
        return
      }
    )
  }
</script>

<div class="d-flex">
  <div>{openedFilepath}</div>
  <!-- todo -->
  <div class="filedrop">
    <DragDrop onDrop={(filepath: string) => open(filepath)} />
    <button onclick={openOnClick}>Open</button>
  </div>
  <button onclick={saveToOverwrite} disabled={!openedFilepath}>Save</button>
  <button onclick={saveAs}>SaveAs</button>
</div>

<style>
  button {
    font-size: 0.9rem;
  }
  .filedrop {
    width: 120px;
    height: 30px;
    margin-right: 1.4rem;
    background-color: yellow;
  }
</style>
