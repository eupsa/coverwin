<script lang="ts">
import { Copy, Minus, Square, X } from "@lucide/svelte";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { onMount } from "svelte";

const win = getCurrentWindow();
let maximized = $state(false);

onMount(() => {
	win.isMaximized().then((m) => (maximized = m));
	const unlisten = win.onResized(
		async () => (maximized = await win.isMaximized()),
	);
	return () => unlisten.then((f) => f());
});

const btn =
	"flex h-full w-[46px] items-center justify-center transition-colors duration-150";
</script>

<header data-tauri-drag-region class="flex h-10 shrink-0 items-center">
	<div data-tauri-drag-region class="flex items-center gap-2.5 pl-4">
		<div
			class="flex size-5 items-center justify-center rounded-[5px] bg-gradient-to-br from-sky-400 to-indigo-500 text-[10px] font-bold text-white shadow-sm"
		>
			c
		</div>
		<span data-tauri-drag-region class="text-xs text-text-2">coverwin</span>
	</div>

	<div class="ml-auto flex h-full">
		<button type="button" aria-label="Minimizar" class="{btn} hover:bg-subtle" onclick={() => win.minimize()}>
			<Minus class="size-4" strokeWidth={1.25} />
		</button>
		<button
			type="button"
			aria-label={maximized ? "Restaurar" : "Maximizar"}
			class="{btn} hover:bg-subtle"
			onclick={() => win.toggleMaximize()}
		>
			{#if maximized}
				<Copy class="size-3.5 -scale-x-100" strokeWidth={1.25} />
			{:else}
				<Square class="size-3.5" strokeWidth={1.25} />
			{/if}
		</button>
		<button type="button" aria-label="Fechar" class="{btn} hover:bg-[#c42b1c] hover:text-white" onclick={() => win.close()}>
			<X class="size-4" strokeWidth={1.25} />
		</button>
	</div>
</header>
