<script lang="ts">
  const { html, onchange }: { html: string; onchange: Function } = $props()
</script>

<textarea
  onchange={(e: Event & { currentTarget: EventTarget & HTMLTextAreaElement }) =>
    onchange(e.currentTarget.value)}>{html}</textarea
>
