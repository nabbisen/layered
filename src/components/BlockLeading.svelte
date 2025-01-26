<script lang="ts">
  const {
    nesting_level,
    heading_level,
    heading_text,
    visibleLevel,
    textOnchange,
    visibleLevelOnChange,
  }: {
    nesting_level: number
    heading_level: number
    heading_text: string
    visibleLevel: number | null
    textOnchange: Function
    visibleLevelOnChange: Function
  } = $props()
</script>

<div class="d-flex">
  <header>
    <nav>
      <button onclick={() => visibleLevelOnChange(nesting_level + 1)}
        >{visibleLevel && nesting_level === visibleLevel - 1 ? '+' : '-'}</button
      >
      <button
        class={!visibleLevel ? 'rotate-90' : nesting_level < visibleLevel - 1 ? 'rotate-90' : ''}
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
        {heading_text}
      </svelte:element>
    {:else}
      <div
        class={`h${heading_level}`}
        onblur={(e: FocusEvent & { currentTarget: EventTarget & HTMLElement }) =>
          textOnchange(e.currentTarget.innerText)}
        contenteditable
      >
        {heading_text}
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
    border: none;
    font-size: 1.1rem;
  }
  footer nav {
    display: none;
    margin-left: -1.2rem;
  }
  nav,
  .content:hover ~ footer nav,
  footer nav:hover {
    display: flex;
  }
</style>
