<script lang="ts">
    import SideElement from "./SideElement.svelte";

    type Message = {
        sender: string,
        content: string,
    };

    export let messages: Message[] = [
        { sender: "Java", content: "Hi" },
        { sender: "Jae", content: "Hello" },
        { sender: "Java", content: "This is a chat" },
        { sender: "Jae", content: "Magical kobold made a chat" },
        { sender: "Java", content: ">.>" },
        { sender: "Jae", content: "beep boop im a synth" },
    ];
</script>

<SideElement>
    {#each messages as msg}
        <span>
            <span class="font-bold">{msg.sender}:</span> <span class="text-sm">{msg.content}</span>
        </span>
    {/each}
</SideElement>
