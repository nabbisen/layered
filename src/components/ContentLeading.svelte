<script lang="ts">
  const {
    text,
    layerLevel,
    hasChildren,
    showsChildren,
    onchange,
    toggleChildren,
  }: {
    text: string
    layerLevel: number
    hasChildren: boolean
    showsChildren: boolean
    onchange: Function
    toggleChildren: Function
  } = $props()

  const textOnBlur = (e: Event & { currentTarget: EventTarget & HTMLElement }) => {
    const textAtBlur = e.currentTarget.innerText
    if (text === textAtBlur) return
    onchange(textAtBlur)
  }

  const navIconStyle = () => {
    return showsChildren ? 'rotate-180' : 'rotate-90'
  }
</script>

{#if hasChildren}
  <div class="d-flex">
    <nav><button onclick={() => toggleChildren()} class={navIconStyle()}>^</button></nav>
    <div role="heading" aria-level={layerLevel} onblur={textOnBlur} contenteditable>
      {text}
    </div>
  </div>
{:else}
  <div class="text" onblur={textOnBlur} contenteditable>{text}</div>
{/if}

<style>
  nav {
    margin-right: 0.7rem;
  }
  nav button {
    font-size: 0.8rem;
    padding: 0;
    background: none;
    border: none;
  }

  .text {
    margin-left: 1.16rem;
  }

  *[contenteditable]:focus {
    background-color: yellow;
  }
</style>
