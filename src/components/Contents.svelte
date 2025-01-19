<script lang="ts">
  import type { ContentType } from '../types'
  import Contents from './Contents.svelte'
  import ContentText from './ContentText.svelte'

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
  const _showsChildren: boolean = showsChildren ?? true

  const _oninput = (index: number, text: string) => {
    contents[index].text = text

    if (onchange) {
      onchange(_indexInParent, contents)
    }
  }

  const _onchange = (index: number, childContents: ContentType[]) => {
    contents[index].children = childContents

    if (onchange) {
      onchange(_indexInParent, contents)
    }
  }
</script>

<div class={`layer layer-${_layerLevel}`}>
  {#each contents as content, i}
    <ContentText
      text={content.text}
      layerLevel={_layerLevel}
      hasChildren={content.children && 0 < content.children.length}
      oninput={(text: string) => _oninput(i, text)}
    />
    {#if content.children}
      <div class={_showsChildren ? '' : 'd-none'}>
        <Contents
          contents={content.children}
          layerLevel={_layerLevel + 1}
          indexInParent={i}
          onchange={_onchange}
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
