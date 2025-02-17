<script>
    import { page } from '$app/state'
    import bgimg from '$lib/images/pc-bg2.png'
</script>

<div id="background" class="relative w-full h-screen overflow-hidden">
    <div class="absolute inset-0 bg-black"></div>
    <div style="background-image: url({bgimg}); filter: blur(8px); transform: scale(1.1);" class="absolute inset-0 bg-cover bg-center opacity-50"></div>
</div>

<div id="main" class="absolute inset-0 flex justify-center items-center z-10">
    <div id="content" class="flex flex-col justify-center items-center bg-white/20 w-1/3 h-2/3">
        <p class="mb-4 text-4xl font-bold leading-none text-gray-900 md:text-5xl lg:text-6xl dark:text-white">Offsides!</p>
        <p class="mb-4 text-xl leading-none text-gray-900 md:text-2xl lg:text-3xl dark:text-white text-center">The page you tried to reach was outside of the rink!</p>
        <p class="mb-4 text-md leading-none text-gray-900 md:text-lg lg:text-xl dark:text-white text-center">If this is a mistake, please contact the refs</p>
        <p>{ page.status }: {page.error?.message}</p>
    </div>
</div>