<script lang="ts">
    import Block from "./Block.svelte";

    let {
        src,
        srcParentItems,
        level,
        index,
        update,
    }: {
        src: undefined | Array<undefined>;
        srcParentItems: Array<undefined>;
        level: number;
        index: number;
        update: Function;
    } = $props();

    let nextLevel = level + 1;

    let dragStartIndex: number | undefined;
    function handleDragStart(index: number) {
        dragStartIndex = index;
    }

    function handleDragOver(event: MouseEvent) {
        event.preventDefault();
    }

    function handleDrop(index: number) {
        console.log(3, index);
        const updatedItems = [...srcParentItems];
        const draggedItem = updatedItems[dragStartIndex!];
        updatedItems.splice(dragStartIndex!, 1); // 元の位置から削除
        updatedItems.splice(index, 0, draggedItem); // 新しい位置に挿入
        update(updatedItems);

        dragStartIndex = undefined;
    }

    function updateItem(index: number, updated: any) {
        src![index] = updated;
    }
</script>

{#if Array.isArray(src)}
    {#each src as item, index}
        <Block
            src={item}
            srcParentItems={src}
            level={nextLevel}
            {index}
            update={(updatedItems: any) => updateItem(index, updatedItems)}
        ></Block>
    {/each}
{:else}
    <div class={`level-${level}`}>
        <span
            class="op"
            role="navigation"
            draggable="true"
            ondragstart={() => handleDragStart(index)}
            ondragover={handleDragOver}
            ondrop={() => handleDrop(index)}>--</span
        >
        <span class="content" contenteditable="true">
            {src}
        </span>
    </div>
{/if}

<style>
    .op {
        cursor: grab;
    }
    .content {
        padding: 0 1.1rem;
        margin-left: 0.8rem;
    }
    .level-1 {
        margin-left: 0.5rem;
    }
    .level-2 {
        margin-left: 1rem;
    }
    .level-3 {
        margin-left: 1.5rem;
    }
    .level-4 {
        margin-left: 2rem;
    }
    .level-5 {
        margin-left: 2.5rem;
    }
    .level-6 {
        margin-left: 3rem;
    }
</style>
