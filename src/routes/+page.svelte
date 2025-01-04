<script lang="ts">
    import Block from "../components/Block.svelte";

    const src: any = ["1", ["1", ["1", ["1", "2", "3"]], "2"], "2"];
</script>

<main class="container">
    {#each src as item, index}
        <Block
            src={item}
            srcParentItems={src}
            level={1}
            {index}
            update={() => {}}
        ></Block>
    {/each}
</main>

<style>
</style>
