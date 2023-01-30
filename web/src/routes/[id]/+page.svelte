<script lang="ts">
    import Chat from './Chat.svelte';
    import Members from './Members.svelte';
    import Queue from './Queue.svelte';
    import YTPlayer from 'yt-player';
    import { onMount } from 'svelte';
    import SideElement from './SideElement.svelte';

    let player: YTPlayer | null = null;

    onMount(() => {
        player = new YTPlayer("#player", {
            modestBranding: true,
        });
        player.load("KWofZgMgSQU");

        player.on("unstarted", () => {
        });
    });
</script>

<main class="px-10 py-10 flex flex-row justify-center flex-row flex-wrap gap-5 max-w-screen-lg lg:max-w-screen m-auto">
    <div class="w-screen-2xl lg:w-screen-xl aspect-video dark:bg-zinc-800 flex justify-center items-center rounded overflow-hidden">
        <span id="player" class="text-3xl dark:text-zinc-600 w-[100%] h-[100%]">Enqueue a Video</span>
    </div>

    <SideElement>
        <h1 class="text-3xl">Debug info</h1>
    </SideElement>

    <Chat />
    <Queue />
    <Members />
</main>
