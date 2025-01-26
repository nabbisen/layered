<script lang="ts">
  const { content, textOnchange }: { content: string; textOnchange: Function } = $props()

  let _content: string = $state(content)

  $effect(() => {
    _content = content
    return
  })
</script>

<textarea
  class="col"
  onchange={(e: Event & { currentTarget: EventTarget & HTMLTextAreaElement }) =>
    textOnchange(e.currentTarget.value)}
  bind:value={_content}
></textarea>
