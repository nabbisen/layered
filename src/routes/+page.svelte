<script lang="ts">
  let content: string = $state('')
  interface ContentAst {
    type: number
    text: string
  }
  let contentAst: ContentAst[] = $derived.by(() => {
    let ret: ContentAst[] = []

    let paragragh = ''
    content.split('\n').forEach((line: string) => {
      const leading = line.split(' ')[0]
      if (leading.match(/^#+$/)) {
        if (paragragh && paragragh.match(/\S/)) {
          ret.push({
            type: 0,
            text: paragragh.replace(/(^\s*|\s*$)/, ''),
          } as ContentAst)
          paragragh = ''
        }
        const leadingLength = leading.length
        ret.push({
          type: leadingLength,
          text: line.substring(leadingLength + 1),
        } as ContentAst)
      } else {
        paragragh = `${paragragh}\n${line}`
      }
    })
    if (paragragh && paragragh.match(/\S/)) {
      ret.push({
        type: 0,
        text: paragragh.replace(/(^\s*|\s*$)/, ''),
      } as ContentAst)
    }

    return ret
  })

  const onchange = (value: string, index: number) => {
    if (contentAst[index].text === value) return

    const updated: ContentAst[] = [...contentAst]
    updated[index].text = value
    let ret: string[] = updated.map((x: ContentAst) => {
      if (0 < x.type) {
        return `${'#'.repeat(x.type)} ${x.text}\n`
      } else {
        return x.text
      }
    })
    content = ret.join('\n')
  }
</script>

<main class="container">
  <div class="d-flex row">
    <textarea bind:value={content}></textarea>
    <div class="col">
      {#each contentAst as line, i}
        {#if 0 < line.type}
          {#if line.type === 1}
            <h1
              onblur={(e: FocusEvent & { currentTarget: EventTarget & HTMLElement }) =>
                onchange(e.currentTarget.innerText, i)}
              contenteditable
            >
              {line.text}
            </h1>
          {:else if line.type === 2}
            <h2
              onblur={(e: FocusEvent & { currentTarget: EventTarget & HTMLElement }) =>
                onchange(e.currentTarget.innerText, i)}
              contenteditable
            >
              {line.text}
            </h2>
          {:else if line.type === 3}
            <h3
              onblur={(e: FocusEvent & { currentTarget: EventTarget & HTMLElement }) =>
                onchange(e.currentTarget.innerText, i)}
              contenteditable
            >
              {line.text}
            </h3>
          {:else if line.type === 4}
            <h4
              onblur={(e: FocusEvent & { currentTarget: EventTarget & HTMLElement }) =>
                onchange(e.currentTarget.innerText, i)}
              contenteditable
            >
              {line.text}
            </h4>
          {:else if line.type === 5}
            <h5
              onblur={(e: FocusEvent & { currentTarget: EventTarget & HTMLElement }) =>
                onchange(e.currentTarget.innerText, i)}
              contenteditable
            >
              {line.text}
            </h5>
          {:else if line.type === 6}
            <h6
              onblur={(e: FocusEvent & { currentTarget: EventTarget & HTMLElement }) =>
                onchange(e.currentTarget.innerText, i)}
              contenteditable
            >
              {line.text}
            </h6>
          {:else}
            <div
              class={`h${line.type}`}
              onblur={(e: FocusEvent & { currentTarget: EventTarget & HTMLElement }) =>
                onchange(e.currentTarget.innerText, i)}
              contenteditable
            >
              {line.text}
            </div>
          {/if}
        {:else}
          <textarea
            onchange={(e: Event & { currentTarget: EventTarget & HTMLTextAreaElement }) =>
              onchange(e.currentTarget.value, i)}>{line.text}</textarea
          >
        {/if}
      {/each}
    </div>
  </div>
</main>

<style>
  .d-flex {
    display: flex !important;
  }

  .row .col {
    width: 50%;
  }

  h1 {
    margin-left: 0;
  }
  h2 {
    margin-left: 0.4rem;
  }
  h3 {
    margin-left: 0.8rem;
  }
  h4 {
    margin-left: 1.2rem;
  }
  h5 {
    margin-left: 1.6rem;
  }
  h6 {
    margin-left: 2rem;
  }
  .h7 {
    margin-left: 2.4rem;
  }
</style>
