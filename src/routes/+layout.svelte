<script lang="ts">
    import type { LayoutData } from "./$types";
    import Header from "./Header.svelte";
    import "../app.css";

    export let data: LayoutData;
</script>

<div class="app bg-gradient-to-br from-black to-blue-400">
    <Header />
    <main>
        <slot />
    </main>
</div>