<script lang="ts">
  import { onMount } from 'svelte'
  import { invoke } from '@tauri-apps/api/core'
  import HeadingNode from './Content/HeadingNode.svelte'
  import ContentNode from './Content/ContentNode.svelte'
  import { type ParsedMarkdown } from '../../types'
  import {
    findMaxHeadingLevel,
    hasNodeChildren,
    isNodeChildrenVisible,
    mapNodeVisibles,
  } from './scripts'
  import '../../styles/editors.css'

  const {
    parsedMarkdowns,
    parsedMarkdownsOnChange,
    contentOnChange,
  }: {
    parsedMarkdowns: ParsedMarkdown[]
    parsedMarkdownsOnChange: (updated: ParsedMarkdown[]) => void
    contentOnChange: (updated: string) => void
  } = $props()

  let _parsedMarkdowns: ParsedMarkdown[] = $state(parsedMarkdowns)
  let maxVisibleNodeLevel: number | null = $state(null)

  let maxHeadingLevel = $derived.by(() => findMaxHeadingLevel(_parsedMarkdowns))
  let maxMaxVisibleNodeLevel = $derived(maxHeadingLevel + 1)

  onMount(() => {
    if (!maxVisibleNodeLevel) {
      maxVisibleNodeLevel = maxMaxVisibleNodeLevel
    }
    _parsedMarkdowns = mapNodeVisibles(_parsedMarkdowns, maxMaxVisibleNodeLevel)
  })

  const nodeTextOnchange = (value: string, index: number, isHeading: boolean) => {
    if (isHeading && _parsedMarkdowns[index].text === value) return

    _parsedMarkdowns[index].text = value
    invoke('compose', { parsedMarkdowns: parsedMarkdowns })
      .then((ret: unknown) => {
        contentOnChange(ret as string)
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
    let ret = _parsedMarkdowns
    ret.splice(index, 0, {
      nodeId: _parsedMarkdowns.length + 1,
      isHeading: isHeading,
      headingLevel: headinLevel,
      text: '',
      parentNodeId: parentNodeId,
      ancestors: ancestors,
      visible: true,
    } as ParsedMarkdown)
    parsedMarkdownsOnChange(ret)
  }

  const addHeadingNode = (
    index: number,
    headingLevel: number,
    nodeParentId: number | null,
    ancestors: number[]
  ) => {
    const nextHeadingIndex = _parsedMarkdowns.findIndex((x, i) => {
      if (i <= index) return false
      return x.isHeading && x.headingLevel < headingLevel
    })
    const found = nextHeadingIndex !== -1
    const headingToAddIndex = found ? nextHeadingIndex : _parsedMarkdowns.length
    addNode(headingToAddIndex, true, headingLevel, nodeParentId, ancestors)
  }

  const removeNode = (nodeId: number) => {
    let ret = _parsedMarkdowns.filter((x) => x.nodeId !== nodeId && !x.ancestors.includes(nodeId))
    parsedMarkdownsOnChange(ret)
  }

  const maxVisibleNodeLevelOnChange = () => {
    let ret = mapNodeVisibles(_parsedMarkdowns, maxVisibleNodeLevel)
    parsedMarkdownsOnChange(ret)
  }
</script>

<div class="container editor">
  <nav class="d-flex">
    <input
      type="number"
      min={1}
      max={maxMaxVisibleNodeLevel}
      bind:value={maxVisibleNodeLevel}
      onchange={maxVisibleNodeLevelOnChange}
    />
  </nav>

  <div class="row">
    <div class="col">
      {#each _parsedMarkdowns as node, i}
        {#if node.visible}
          <div class="line">
            <div class={`nested nest-${node.headingLevel}`}>
              {#if node.isHeading}
                <HeadingNode
                  headingLevel={node.headingLevel}
                  hasChildren={hasNodeChildren(node.nodeId, _parsedMarkdowns)}
                  childrenVisible={isNodeChildrenVisible(node.nodeId, _parsedMarkdowns)}
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
                    _parsedMarkdowns = mapNodeVisibles(_parsedMarkdowns, maxVisibleNodeLevel)
                  }}
                  childrenVisibleOnChange={(updated: boolean) => {
                    _parsedMarkdowns = _parsedMarkdowns.map((x) => {
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
  </div>
</div>
