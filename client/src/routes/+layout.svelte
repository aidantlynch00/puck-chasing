<script>
    let { children } = $props();
    import { page } from '$app/stores'
    import bgimg from '$lib/images/pc-bg2.png'
    import "../app.css";
</script>

{#if $page.url.pathname !== '/'}
    <div id="background" class="relative w-full h-screen overflow-hidden">
        <div class="absolute inset-0 bg-black"></div>
        <div style="background-image: url({bgimg}); filter: blur(8px); transform: scale(1.1);" class="absolute inset-0 bg-cover bg-center opacity-50"></div>
    </div>
{/if}

{@render children()}

<style>
</style>