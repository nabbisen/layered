<script lang="ts">
  import { onMount } from 'svelte'
  import { invoke } from '@tauri-apps/api/core'
  import HeadingNode from './Content/HeadingNode.svelte'
  import ContentNode from './Content/ContentNode.svelte'
  import RawContent from './Content/RawContent.svelte'
  import FileHandler from './Helpers/FileHandler.svelte'
  import { type ParsedMarkdown } from './types'
  import {
    findMaxHeadingLevel,
    hasNodeChildren,
    isNodeChildrenVisible,
    mapNodeVisibles,
  } from './scripts'
  import './styles.css'

  let content: string = $state('')
  let parsedMarkdowns: ParsedMarkdown[] = $state([])
  let maxVisibleNodeLevel: number | null = $state(null)

  let maxHeadingLevel = $derived.by(() => findMaxHeadingLevel(parsedMarkdowns))
  let maxMaxVisibleNodeLevel = $derived(maxHeadingLevel + 1)

  onMount(() => {
    // todo dev dummy
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

  const rawTextOnchange = (value: string) => {
    content = value
    parseMarkdownText(value)
  }

  const parseMarkdownText = (markdownText: string) => {
    invoke('parse', { markdownText: markdownText })
      .then((ret: unknown) => {
        console.log(ret) // todo
        parsedMarkdowns = ret as ParsedMarkdown[]
        if (!maxVisibleNodeLevel) {
          maxVisibleNodeLevel = maxMaxVisibleNodeLevel
        }
        parsedMarkdowns = mapNodeVisibles(parsedMarkdowns, maxMaxVisibleNodeLevel)
      })
      .catch((error: unknown) => {
        console.error(error)
        return
      })
  }

  const nodeTextOnchange = (value: string, index: number, isHeading: boolean) => {
    if (isHeading && parsedMarkdowns[index].text === value) return

    parsedMarkdowns[index].text = value
    invoke('compose', { parsedMarkdowns: parsedMarkdowns })
      .then((ret: unknown) => {
        content = ret as string
      })
      .catch((error: unknown) => {
        console.error(error)
        return
      })
  }

  const addNode = (
    index: number,
    isHeading: boolean,
    headinLevel: number,
    parentNodeId: number | null,
    ancestors: number[]
  ) => {
    parsedMarkdowns.splice(index, 0, {
      nodeId: parsedMarkdowns.length + 1,
      isHeading: isHeading,
      headingLevel: headinLevel,
      text: '',
      parentNodeId: parentNodeId,
      ancestors: ancestors,
      visible: true,
    } as ParsedMarkdown)
    parsedMarkdowns = parsedMarkdowns
  }

  const addHeadingNode = (
    index: number,
    headingLevel: number,
    nodeParentId: number | null,
    ancestors: number[]
  ) => {
    const nextHeadingIndex = parsedMarkdowns.findIndex((x, i) => {
      if (i <= index) return false
      return x.isHeading && x.headingLevel < headingLevel
    })
    const found = nextHeadingIndex !== -1
    const headingToAddIndex = found ? nextHeadingIndex : parsedMarkdowns.length
    addNode(headingToAddIndex, true, headingLevel, nodeParentId, ancestors)
  }

  const removeNode = (nodeId: number) => {
    parsedMarkdowns = parsedMarkdowns.filter(
      (x) => x.nodeId !== nodeId && !x.ancestors.includes(nodeId)
    )
  }

  type EditorLayout = 'raw' | 'both' | 'layers'
  const EDITOR_LAYOUTS: EditorLayout[] = ['raw', 'both', 'layers']
  let activeEditor: EditorLayout = $state('layers')

  const isRawEditorVisible = (): boolean => {
    const matchers: EditorLayout[] = ['raw', 'both']
    return matchers.includes(activeEditor)
  }

  const isLayersEditorVisible = (): boolean => {
    const matchers: EditorLayout[] = ['layers', 'both']
    return matchers.includes(activeEditor)
  }
</script>

<main class="container editor">
  <nav class="d-flex">
    <input type="number" min={1} max={maxMaxVisibleNodeLevel} bind:value={maxVisibleNodeLevel} />
    <div class="d-flex">
      {#each EDITOR_LAYOUTS as editorLayout}
        <label
          ><input
            type="radio"
            name="active-editor"
            bind:group={activeEditor}
            value={editorLayout}
          />{editorLayout}</label
        >
      {/each}
    </div>
  </nav>
  <div class="row">
    {#if isRawEditorVisible()}
      <div class="col">
        <RawContent {content} textOnchange={rawTextOnchange} />
      </div>
    {/if}
    {#if isLayersEditorVisible()}
      <div class="col">
        {#each parsedMarkdowns as node, i}
          {#if node.visible}
            <div class="line">
              <div class={`nested nest-${node.headingLevel}`}>
                {#if node.isHeading}
                  <HeadingNode
                    headingLevel={node.headingLevel}
                    hasChildren={hasNodeChildren(node.nodeId, parsedMarkdowns)}
                    childrenVisible={isNodeChildrenVisible(node.nodeId, parsedMarkdowns)}
                    text={node.text}
                    textOnchange={(value: string) => {
                      nodeTextOnchange(value, i, true)
                    }}
                    maxVisibleNodeLevelOnChange={() => {
                      if (maxVisibleNodeLevel === node.headingLevel) {
                        maxVisibleNodeLevel = maxMaxVisibleNodeLevel
                      } else {
                        maxVisibleNodeLevel = node.headingLevel
                      }
                      parsedMarkdowns = mapNodeVisibles(parsedMarkdowns, maxVisibleNodeLevel)
                    }}
                    childrenVisibleOnChange={(updated: boolean) => {
                      parsedMarkdowns = parsedMarkdowns.map((x) => {
                        const mod = x
                        if (mod.ancestors.includes(node.nodeId)) {
                          mod.visible = updated
                        }
                        return mod
                      })
                    }}
                    addSiblingHeading={() =>
                      addHeadingNode(i, node.headingLevel, node.parentNodeId, node.ancestors)}
                    addChildHeading={() =>
                      addHeadingNode(i, node.headingLevel + 1, node.nodeId, [
                        ...node.ancestors,
                        node.nodeId,
                      ])}
                    addChildContent={() =>
                      addNode(i + 1, false, node.headingLevel, node.nodeId, [
                        ...node.ancestors,
                        node.nodeId,
                      ])}
                    remove={() => removeNode(node.nodeId)}
                  />
                {:else}
                  <ContentNode
                    text={node.text}
                    textOnchange={(value: string) => nodeTextOnchange(value, i, false)}
                  />
                {/if}
              </div>
            </div>
          {/if}
        {/each}
      </div>
    {/if}
  </div>
</main>
<FileHandler
  {parsedMarkdowns}
  rawContentOnChange={(rawContent: string) => {
    content = rawContent
    parseMarkdownText(content)
  }}
/>
