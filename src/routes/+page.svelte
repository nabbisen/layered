<script lang="ts">
  import { onMount } from 'svelte'
  import { invoke } from '@tauri-apps/api/core'
  import BlockLeading from '../components/BlockLeading.svelte'
  import BlockContent from '../components/BlockContent.svelte'

  onMount(() => {
    invoke('ready', {})
      .then((ret: unknown) => {
        content = ret as string
        parseMarkdownText(content)
      })
      .catch((error: unknown) => {
        console.error(error)
        return
      })
  })

  const markdownTextOnchange = (
    e: Event & { currentTarget: EventTarget & HTMLTextAreaElement }
  ) => {
    parseMarkdownText(e.currentTarget.value)
  }

  const parseMarkdownText = (markdownText: string) => {
    invoke('parse', { markdownText: markdownText })
      .then((ret: unknown) => {
        console.log(ret)
        parsedMarkdowns = ret as ParsedMarkdown[]
        changeVisibility()
      })
      .catch((error: unknown) => {
        console.error(error)
        return
      })
  }

  let visibleLevel: number | null = $state(null)
  const changeVisibility = () => {
    parsedMarkdowns = parsedMarkdowns.map((x) => {
      const updateVisible = x
      if (!visibleLevel || Number.isNaN(visibleLevel) || Number(visibleLevel) <= 0) {
        updateVisible.visible = true
        return updateVisible
      }
      updateVisible.visible = updateVisible.heading_level
        ? updateVisible.heading_level <= Number(visibleLevel)
        : false
      return updateVisible
    })
  }

  const visibleOnchange = (e: Event & { currentTarget: EventTarget & HTMLInputElement }) => {
    visibleLevel = Number.isNaN(e.currentTarget.value) ? null : Number(e.currentTarget.value)
    changeVisibility()
  }

  let content: string = $state('')
  interface ParsedMarkdown {
    node_id: number
    ancestors: number[]
    nesting_level: number
    heading_level: number | null
    heading_text: string | null
    html: string | null
    visible: boolean
  }
  let parsedMarkdowns: ParsedMarkdown[] = $state([])
  let maxNestingLevel = $derived.by(() => {
    let maxLevel: number = 0
    parsedMarkdowns.forEach((x: ParsedMarkdown) => {
      if (maxLevel < x.nesting_level) maxLevel = x.nesting_level
    })
    return maxLevel
  })
  // interface ContentAst {
  //   type: number
  //   text: string
  // }
  // let contentAst: ContentAst[] = $derived.by(() => {
  //   let ret: ContentAst[] = []

  //   let paragragh = ''
  //   content.split('\n').forEach((line: string) => {
  //     const leading = line.split(' ')[0]
  //     if (leading.match(/^#+$/)) {
  //       if (paragragh && paragragh.match(/\S/)) {
  //         ret.push({
  //           type: 0,
  //           text: paragragh.replace(/(^\s*|\s*$)/, ''),
  //         } as ContentAst)
  //         paragragh = ''
  //       }
  //       const leadingLength = leading.length
  //       ret.push({
  //         type: leadingLength,
  //         text: line.substring(leadingLength + 1),
  //       } as ContentAst)
  //     } else {
  //       paragragh = `${paragragh}\n${line}`
  //     }
  //   })
  //   if (paragragh && paragragh.match(/\S/)) {
  //     ret.push({
  //       type: 0,
  //       text: paragragh.replace(/(^\s*|\s*$)/, ''),
  //     } as ContentAst)
  //   }

  //   return ret
  // })

  const onchange = (value: string, index: number, isHeading: boolean) => {
    if (isHeading && parsedMarkdowns[index].heading_text === value) return

    // const updated: ContentAst[] = [...contentAst]
    // updated[index].text = value
    // let ret: string[] = updated.map((x: ContentAst) => {
    //   if (0 < x.type) {
    //     return `${'#'.repeat(x.type)} ${x.text}\n`
    //   } else {
    //     return x.text
    //   }
    // })
    // content = ret.join('\n')
  }
</script>

<main class="container">
  <input
    type="number"
    min="0"
    max={maxNestingLevel}
    onchange={visibleOnchange}
    bind:value={visibleLevel}
  />
  <div class="d-flex row">
    <textarea class="col" onchange={markdownTextOnchange} bind:value={content}></textarea>
    <div class="col">
      {#each parsedMarkdowns as block, i}
        <div class={`nested nest-${block.nesting_level}`}>
          {#if block.visible}
            {#if block.heading_level && 0 < block.heading_level}
              <BlockLeading
                heading_level={block.heading_level}
                heading_text={block.heading_text ?? ''}
                onchange={(value: string) => onchange(value, i, true)}
              />
            {:else}
              <BlockContent
                html={block.html ?? ''}
                onchange={(value: string) => onchange(value, i, false)}
              />
            {/if}
          {/if}
        </div>
      {/each}
    </div>
  </div>
</main>

<style>
  .d-flex {
    display: flex !important;
  }

  .row .col {
    width: 50%;
  }

  .nested {
    padding-left: 4.4rem;
  }
  .nested.nest-0 {
    padding-left: 0;
  }
  .nested.nest-1 {
    padding-left: 0.9rem;
  }
  .nested.nest-2 {
    padding-left: 1.8rem;
  }
  .nested.nest-3 {
    padding-left: 2.7rem;
  }
  .nested.nest-4 {
    padding-left: 3.6rem;
  }
</style>
