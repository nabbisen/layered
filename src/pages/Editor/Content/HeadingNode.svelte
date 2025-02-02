<script lang="ts">
  const {
    headingLevel,
    text,
    childrenVisible,
    textOnchange,
    maxVisibleNodeLevelOnChange,
    childrenVisibleOnChange,
    addSiblingHeading,
    addChildHeading,
    addChildContent,
    remove,
  }: {
    headingLevel: number
    text: string
    childrenVisible: boolean | null
    textOnchange: Function
    maxVisibleNodeLevelOnChange: Function
    childrenVisibleOnChange: Function
    addSiblingHeading: Function
    addChildHeading: Function
    addChildContent: Function
    remove: Function
  } = $props()
</script>

<div class="d-flex">
  <header>
    <nav>
      <button onclick={() => maxVisibleNodeLevelOnChange()}>{!childrenVisible ? '+' : '-'}</button>
      <button
        class={childrenVisible ? 'rotate-90' : ''}
        onclick={() => childrenVisibleOnChange(!childrenVisible)}>></button
      >
    </nav>
  </header>
  <div class="content">
    {#if headingLevel <= 6}
      <svelte:element
        this={`h${headingLevel}`}
        onblur={(e: FocusEvent & { currentTarget: EventTarget & HTMLElement }) =>
          textOnchange(e.currentTarget.innerText)}
        contenteditable
      >
        {text}
      </svelte:element>
    {:else}
      <div
        class={`h${headingLevel}`}
        onblur={(e: FocusEvent & { currentTarget: EventTarget & HTMLElement }) =>
          textOnchange(e.currentTarget.innerText)}
        contenteditable
      >
        {text}
      </div>
    {/if}
  </div>
  <footer>
    <nav>
      <button onclick={() => addSiblingHeading()}>+-</button>
      <button onclick={() => addChildHeading()}>+/</button>
      <button onclick={() => addChildContent()}>+|</button>
      <button onclick={() => remove()}>--</button>
    </nav>
  </footer>
</div>

<style>
  h1,
  h2,
  h3,
  h4,
  h5,
  h6 {
    padding: 0;
    margin: 0;
  }

  nav {
    padding-right: 1.1rem;
  }
  nav button {
    background: none;
    color: var(--theme-color);
    border: none;
    font-size: 1.1rem;
    border-radius: 0.22rem;
    border: 0.04rem solid transparent;
  }
  nav button:hover {
    opacity: 0.7;
    border-color: silver;
  }
  footer nav {
    display: none;
  }
  nav,
  .content:hover ~ footer nav,
  footer nav:hover {
    display: flex;
  }
</style>
