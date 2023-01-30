<script lang="ts">
    import { onMount } from "svelte";
    import "virtual:windi.css";
    import '../app.pcss';
    import { init } from "../dark";

    onMount(init);
</script>

<slot />
