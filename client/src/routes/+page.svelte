<script lang='ts'>
    import bgimg from '$lib/images/pc-bg.png'
    import SearchBar from '$lib/components/SearchBar.svelte';
</script>

<div id="background" class="relative w-full h-screen overflow-hidden">
    <div class="absolute inset-0 bg-black"></div>
    <div style="background-image: url({bgimg}); filter: blur(8px); transform: scale(1.1);" class="absolute inset-0 bg-cover bg-center opacity-50"></div>
</div>

<!-- <div id="main" class="absolute inset-0 flex justify-center items-center z-10"> -->
<!-- <div id="main" class="fixed inset-0 flex justify-center items-center z-10 overflow-y-auto"> -->
<div id="main" class="fixed inset-x-0 top-0 flex justify-center z-10">
    <div class="mt-50 max-h-screen">
        <h1 class="mb-4 text-4xl font-extrabold leading-none tracking-tight text-gray-900 md:text-5xl lg:text-6xl dark:text-white">Puck Chasing</h1>
        <SearchBar />
    </div>
</div>
