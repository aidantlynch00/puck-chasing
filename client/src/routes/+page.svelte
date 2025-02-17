<script>
    import bgimg from '$lib/images/pc-bg.png'
</script>

<div id="background" class="relative w-full h-screen overflow-hidden">
    <div class="absolute inset-0 bg-black"></div>
    <div style="background-image: url({bgimg}); filter: blur(8px); transform: scale(1.1);" class="absolute inset-0 bg-cover bg-center opacity-50"></div>
</div>

<div id="main" class="absolute inset-0 flex justify-center items-center z-10">
    <div class="mt-2">
        <h1 class="mb-4 text-4xl font-extrabold leading-none tracking-tight text-gray-900 md:text-5xl lg:text-6xl dark:text-white">Puck Chasing</h1>
        <div class="flex items-center rounded-md bg-white pl-3 outline-1 -outline-offset-1 outline-gray-300 has-[input:focus-within]:outline-2 has-[input:focus-within]:-outline-offset-2 has-[input:focus-within]:outline-blue-500">
            <div class="shrink-0 text-base text-gray-500 select-none sm:text-sm/6">
                <svg class="h-5 w-5 text-gray-500"  width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                    <path stroke="none" d="M0 0h24v24H0z"/>
                    <circle cx="10" cy="10" r="7" />
                    <line x1="21" y1="21" x2="15" y2="15" />
                </svg>
            </div>
            <input type="text" name="search" class="block min-w-0 grow py-1.5 pr-3 pl-1 text-base text-gray-900 placeholder:text-gray-400 focus:outline-none sm:text-sm/6" placeholder="SlapID or Username">
        </div>
    </div>
</div>
