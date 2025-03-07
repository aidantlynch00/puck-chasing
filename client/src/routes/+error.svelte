<script>
    import { page } from '$app/state'

    const errorMessages = [
        "You've been sent to the penalty box!",
        "The goalie has blocked your request!",
        "The refs blew the whistle on that request!",
        "You've been called offsides!"
    ]
</script>

<div id="main" class="absolute inset-0 flex justify-center items-center z-10">
    <div id="content" class="flex flex-col justify-center items-center bg-white/20 w-1/3 h-2/3">
        <p class="mb-4 text-4xl font-bold leading-none text-gray-900 md:text-5xl lg:text-6xl dark:text-white">Oh no!</p>
        <p class="mb-4 text-xl leading-none text-gray-900 md:text-2xl lg:text-3xl dark:text-white text-center">{errorMessages[Math.floor(Math.random() * errorMessages.length)]}</p>
        <p class="mb-4 text-md leading-none text-gray-900 md:text-lg lg:text-xl dark:text-white text-center">If this is a mistake, please contact the site administrators</p>
        <p>{ page.status }: {page.error?.message}</p>
    </div>
</div>