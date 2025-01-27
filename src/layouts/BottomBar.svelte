<script lang="ts">
  let isTopBarVisible: boolean = $state(false)

  type UserFontFamily = 'Monospace1' | 'SansSerif1' | 'SansSerif2' | 'Serif1'
  const USER_FONT_FAMILIES: UserFontFamily[] = ['Monospace1', 'SansSerif1', 'SansSerif2', 'Serif1']
  const APP_FONT_FAMILY_CLASS_PREFIX: string = 'font-family-'
  const EDITOR_FONT_FAMILY_CLASS_PREFIX: string = 'editor-font-family-'

  let appFontFamily: UserFontFamily = $state('SansSerif1')
  let editorFontFamily: UserFontFamily = $state('SansSerif1')

  const appFontFamilyOnchange = () => {
    handleFontFamily(appFontFamily, APP_FONT_FAMILY_CLASS_PREFIX)
  }
  const editorFontFamilyOnchange = () => {
    handleFontFamily(editorFontFamily, EDITOR_FONT_FAMILY_CLASS_PREFIX)
  }
  const handleFontFamily = (activeClass: string, classPrefix: string) => {
    const class_list = document.documentElement.classList
    USER_FONT_FAMILIES.forEach((x) => {
      class_list.remove(classPrefix + x)
    })
    class_list.add(classPrefix + activeClass)
  }

  let appFontSize: number = $state(0.93)
  let editorFontSize: number = $state(1.0)
  const appFontSizeOnchange = () => {
    handleFontSize(appFontSize, '--theme-font-size')
  }
  const editorFontSizeOnchange = () => {
    handleFontSize(appFontSize, '--theme-editor-font-size')
  }
  const handleFontSize = (fontSize: number, propertyName: string) => {
    document.documentElement.style.setProperty(propertyName, `${fontSize}rem`)
  }
</script>

<label>|||<input type="checkbox" bind:checked={isTopBarVisible} /></label>
{#if isTopBarVisible}
  <div class="d-flex">
    <label
      >main font size<input
        type="number"
        min="0.5"
        step="0.1"
        max="5.0"
        bind:value={appFontSize}
        onchange={appFontSizeOnchange}
      /></label
    >
    <label
      >editor font size<input
        type="number"
        min="0.5"
        step="0.1"
        max="5.0"
        bind:value={editorFontSize}
        onchange={editorFontSizeOnchange}
      /></label
    >
  </div>
  <div class="d-flex">
    <div>main font family</div>
    {#each USER_FONT_FAMILIES as userFontFamily}
      <label>
        <input
          type="radio"
          name="app-font-family"
          value={userFontFamily}
          bind:group={appFontFamily}
          onchange={appFontFamilyOnchange}
        />{userFontFamily}
      </label>
    {/each}
  </div>
  <div class="d-flex">
    <div>editor font family</div>
    {#each USER_FONT_FAMILIES as userFontFamily}
      <label>
        <input
          type="radio"
          name="editor-font-family"
          value={userFontFamily}
          bind:group={editorFontFamily}
          onchange={editorFontFamilyOnchange}
        />{userFontFamily}
      </label>
    {/each}
  </div>
{/if}
