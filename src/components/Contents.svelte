<script lang="ts">
  import type { ContentType } from '../types'
  import Contents from './Contents.svelte'
  import ContentLeading from './ContentLeading.svelte'

  const MIN_LAYER_LEVEL: number = 4

  const {
    contents,
    layerLevel,
    indexInParent,
    showsChildren,
    onchange,
  }: {
    contents: ContentType[]
    layerLevel?: number
    indexInParent?: number
    showsChildren?: boolean
    onchange?: Function
  } = $props()

  const _layerLevel: number = layerLevel ?? MIN_LAYER_LEVEL
  const _indexInParent: number = indexInParent ?? 0

  let _showsEachChild: boolean[] = $state(
    Array.from({ length: contents.length }).map((_) => showsChildren ?? true)
  )

  const hasChildren = (children: ContentType[] | undefined): boolean => {
    return (children && 0 < children.length) as boolean
  }

  const contentLeadingOnchange = (index: number, text: string) => {
    contents[index].text = text

    if (onchange) {
      onchange(_indexInParent, contents)
    }
  }

  const contentLeadingToggleChildren = (index: number) => {
    _showsEachChild[index] = !_showsEachChild[index]
  }

  const contentsOnchange = (index: number, childContents: ContentType[]) => {
    contents[index].children = childContents

    if (onchange) {
      onchange(_indexInParent, contents)
    }
  }
</script>

<div class={`layer layer-${_layerLevel}`}>
  {#each contents as content, i}
    <ContentLeading
      text={content.text}
      layerLevel={_layerLevel}
      hasChildren={hasChildren(content.children)}
      showsChildren={_showsEachChild[i]}
      onchange={(text: string) => contentLeadingOnchange(i, text)}
      toggleChildren={() => contentLeadingToggleChildren(i)}
    />
    {#if content.children}
      <div class={_showsEachChild[i] ? '' : 'd-none'}>
        <Contents
          contents={content.children}
          layerLevel={_layerLevel + 1}
          indexInParent={i}
          onchange={contentsOnchange}
        />
      </div>
    {/if}
  {/each}
</div>

<style>
  .layer.layer-4 {
    margin-left: 0;
  }
  .layer.layer-5,
  .layer.layer-6,
  .layer.layer-7,
  .layer.layer-8 {
    margin-left: 0.9rem;
  }
  .layer {
    margin-left: 0.3rem;
  }
</style>
