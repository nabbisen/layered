<script lang="ts">
  import { type ContentType } from '../types'
  import Contents from '../components/Contents.svelte'

  const gen_children = (
    layer: number,
    number: number,
    children?: ContentType[][]
  ): ContentType[] => {
    return Array.from({ length: number }).map((_, i) => {
      return {
        text: `L${layer}-${i + 1}`,
        children: children && i < children.length ? children[i] : undefined,
      } as ContentType
    })
  }

  const l6_1: ContentType[] = gen_children(6, 1)
  const l5_1: ContentType[] = gen_children(5, 1, [l6_1])
  const l5_2: ContentType[] = gen_children(5, 1)
  const l4_1: ContentType[] = gen_children(4, 1, [l5_1, l5_2])
  const l4_2: ContentType[] = gen_children(4, 2)
  const l3_1: ContentType[] = gen_children(3, 2, [l4_1, l4_2])
  const l2_1: ContentType[] = gen_children(2, 2, [l3_1])
  const l1: ContentType[] = gen_children(1, 2, [l2_1])

  let showsDump: boolean = $state(false)
</script>

<button
  onclick={() => {
    showsDump = true
    setTimeout(() => {
      showsDump = false
    }, 3000)
  }}>!!!</button
>
{#if showsDump}
  <div
    style="position: fixed; left: 1rem; top: 1rem; width: 100vw; height: auto; padding: 0; margin: 0; backgrond-color: #ffffffb7;"
  >
    <pre>
      {JSON.stringify(l1).split(',').join(',\n')}
    </pre>
  </div>
{/if}

<Contents contents={l1} />
