<script lang="ts">
  const {
    is_heading,
    heading_level,
    text,
    visibleLevel,
    textOnchange,
    visibleLevelOnChange,
  }: {
    is_heading: boolean
    heading_level: number
    text: string
    visibleLevel: number | null
    textOnchange: Function
    visibleLevelOnChange: Function
  } = $props()

  const hasChildrenNested = () => {
    return visibleLevel && heading_level === visibleLevel
  }
</script>

<div class="d-flex">
  <header>
    <nav>
      <button onclick={() => visibleLevelOnChange(heading_level)}
        >{hasChildrenNested() ? '+' : '-'}</button
      >
      <button class={!visibleLevel ? 'rotate-90' : heading_level < visibleLevel ? 'rotate-90' : ''}
        >></button
      >
    </nav>
  </header>
  <div class="content">
    {#if heading_level <= 6}
      <svelte:element
        this={`h${heading_level}`}
        onblur={(e: FocusEvent & { currentTarget: EventTarget & HTMLElement }) =>
          textOnchange(e.currentTarget.innerText)}
        contenteditable
      >
        {text}
      </svelte:element>
    {:else}
      <div
        class={`h${heading_level}`}
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
      <button>+-</button>
      <button>+|</button>
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
    padding: 0 1.1rem;
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
