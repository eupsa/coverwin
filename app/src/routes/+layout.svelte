<script lang="ts">
import "../app.css";
import TitleBar from "../lib/TitleBar.svelte";

let { children } = $props();
</script>

<div class="flex h-screen flex-col">
	<TitleBar />
	{@render children()}
</div>
