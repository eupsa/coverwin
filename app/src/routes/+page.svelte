<script lang="ts">
import {
	Check,
	FolderOpen,
	ImagePlus,
	LoaderCircle,
	Plus,
	Sparkles,
	Trash2,
	X,
} from "@lucide/svelte";
import { Channel, convertFileSrc, invoke } from "@tauri-apps/api/core";
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { open } from "@tauri-apps/plugin-dialog";
import { RadioGroup, Slider } from "bits-ui";
import { onMount } from "svelte";
import { flip } from "svelte/animate";
import { cubicOut, quintOut } from "svelte/easing";
import { fade, fly, scale } from "svelte/transition";

type Progress =
	| {
			status: "done";
			input: string;
			output: string;
			size: number;
			inputSize: number;
	  }
	| { status: "failed"; input: string; error: string };
type Item = {
	path: string;
	state: "pending" | "running" | "done" | "failed";
	error?: string;
	output?: string;
	size?: number;
	inputSize?: number;
};

// mesmas extensões que o core lê (AVIF não: decode desabilitado)
const INPUT_EXTS = [
	"png",
	"jpg",
	"jpeg",
	"webp",
	"bmp",
	"gif",
	"tif",
	"tiff",
	"ico",
];
const FORMATS = [
	{ value: "webp", hint: "Web, leve" },
	{ value: "avif", hint: "Menor tamanho" },
	{ value: "jpg", hint: "Fotos" },
	{ value: "png", hint: "Sem perdas" },
	{ value: "gif", hint: "Compatível" },
	{ value: "bmp", hint: "Sem compressão" },
	{ value: "tiff", hint: "Impressão" },
	{ value: "ico", hint: "Ícones" },
];

let items = $state<Item[]>([]);
let format = $state("webp");
let quality = $state(85);
let outDir = $state<string | null>(null);
let dragging = $state(false);
let busy = $state(false);

const name = (p: string) => p.split(/[\\/]/).pop() ?? p;
const ext = (p: string) => p.split(".").pop()?.toLowerCase() ?? "";
const kb = (n: number) =>
	n < 1024 * 1024
		? `${Math.max(1, Math.round(n / 1024))} KB`
		: `${(n / 1024 / 1024).toFixed(1)} MB`;

function add(paths: string[]) {
	const known = new Set(items.map((i) => i.path));
	for (const path of paths)
		if (INPUT_EXTS.includes(ext(path)) && !known.has(path))
			items.push({ path, state: "pending" });
}

async function pickFiles() {
	const picked = await open({
		multiple: true,
		filters: [{ name: "Imagens", extensions: INPUT_EXTS }],
	});
	if (picked) add(picked);
}

async function pickDir() {
	const picked = await open({ directory: true });
	if (picked) outDir = picked;
}

async function convert() {
	const todo = items.filter((i) => i.state !== "done" && i.state !== "running");
	if (!todo.length) return;
	busy = true;
	for (const i of todo)
		Object.assign(i, { state: "running", error: undefined });

	const channel = new Channel<Progress>();
	channel.onmessage = (msg) => {
		const item = items.find((i) => i.path === msg.input);
		if (!item) return;
		if (msg.status === "done")
			Object.assign(item, {
				state: "done",
				output: msg.output,
				size: msg.size,
				inputSize: msg.inputSize,
			});
		else Object.assign(item, { state: "failed", error: msg.error });
	};
	try {
		await invoke("convert", {
			files: todo.map((i) => i.path),
			format,
			quality,
			outDir,
			channel,
		});
	} catch (e) {
		for (const i of todo)
			if (i.state === "running")
				Object.assign(i, { state: "failed", error: String(e) });
	}
	busy = false;
}

onMount(() => {
	const unlisten = getCurrentWebview().onDragDropEvent((e) => {
		if (e.payload.type === "enter" || e.payload.type === "over")
			dragging = true;
		else {
			dragging = false;
			if (e.payload.type === "drop") add(e.payload.paths);
		}
	});
	return () => unlisten.then((f) => f());
});

const usesQuality = $derived(["jpg", "webp", "avif"].includes(format));
const pending = $derived(
	items.filter((i) => i.state === "pending" || i.state === "failed").length,
);
const done = $derived(items.filter((i) => i.state === "done"));
const progress = $derived(
	items.length
		? (done.length + items.filter((i) => i.state === "failed").length) /
				items.length
		: 0,
);
const saved = $derived(
	done.reduce((s, i) => s + (i.inputSize ?? 0) - (i.size ?? 0), 0),
);
</script>

<div class="flex min-h-0 flex-1">
	<!-- Fila / área de soltar -->
	<section class="relative flex min-w-0 flex-1 flex-col pb-4 pl-4">
		{#if items.length === 0}
			<button
				type="button"
				onclick={pickFiles}
				in:fade={{ duration: 200 }}
				class="group flex flex-1 flex-col items-center justify-center gap-4 rounded-(--radius-card) border border-dashed border-stroke bg-layer transition-all duration-300 ease-(--ease-fluent) hover:border-accent/60 hover:bg-card"
			>
				<div
					class="flex size-20 items-center justify-center rounded-2xl bg-gradient-to-br from-sky-400/20 to-indigo-500/20 text-accent [animation:float_3s_ease-in-out_infinite] group-hover:scale-110 transition-transform duration-300 ease-(--ease-fluent)"
				>
					<ImagePlus class="size-9" strokeWidth={1.5} />
				</div>
				<div class="text-center">
					<p class="font-display text-xl font-semibold">Arraste imagens para cá</p>
					<p class="mt-1 text-text-2">ou clique para escolher · PNG, JPG, WebP, GIF, BMP, TIFF, ICO</p>
				</div>
			</button>
		{:else}
			<div class="mb-3 flex items-center gap-2" in:fade={{ duration: 150 }}>
				<h1 class="font-display text-lg font-semibold">
					{items.length}
					{items.length === 1 ? "imagem" : "imagens"}
				</h1>
				{#if saved > 0}
					<span
						in:scale={{ start: 0.8, duration: 250, easing: quintOut }}
						class="flex items-center gap-1 rounded-full bg-success/15 px-2 py-0.5 text-xs font-medium text-success"
					>
						<Sparkles class="size-3" /> {kb(saved)} economizados
					</span>
				{/if}
				<div class="ml-auto flex gap-1">
					<button
						type="button"
						onclick={pickFiles}
						class="flex h-8 items-center gap-1.5 rounded-(--radius-control) px-3 text-text-2 transition-colors hover:bg-subtle hover:text-text"
					>
						<Plus class="size-4" /> Adicionar
					</button>
					<button
						type="button"
						onclick={() => (items = [])}
						disabled={busy}
						class="flex h-8 items-center gap-1.5 rounded-(--radius-control) px-3 text-text-2 transition-colors hover:bg-subtle hover:text-text disabled:opacity-40"
					>
						<Trash2 class="size-4" /> Limpar
					</button>
				</div>
			</div>

			<div class="-mr-1 min-h-0 flex-1 overflow-y-auto pr-1">
				<ul class="grid grid-cols-[repeat(auto-fill,minmax(150px,1fr))] gap-3">
					{#each items as item, i (item.path)}
						<li
							animate:flip={{ duration: 300, easing: quintOut }}
							in:fly={{ y: 12, duration: 300, delay: Math.min(i, 12) * 30, easing: cubicOut }}
							out:scale={{ start: 0.9, duration: 180 }}
							class="group relative overflow-hidden rounded-(--radius-card) border border-stroke bg-card shadow-sm transition-all duration-200 ease-(--ease-fluent) hover:-translate-y-0.5 hover:shadow-md"
							title={item.path}
						>
							<div class="checker relative aspect-square overflow-hidden">
								<img
									src={convertFileSrc(item.path)}
									alt=""
									loading="lazy"
									class="size-full object-contain transition-transform duration-500 ease-(--ease-fluent) group-hover:scale-105
										{item.state === 'running' ? 'opacity-50 blur-[2px]' : ''}"
								/>
								<span class="absolute top-2 left-2 rounded bg-black/55 px-1.5 py-0.5 text-[10px] font-semibold tracking-wide text-white uppercase backdrop-blur-sm">
									{ext(item.path)}{#if item.output}&nbsp;→ {ext(item.output)}{/if}
								</span>

								{#if item.state === "running"}
									<div class="absolute inset-0 flex items-center justify-center" transition:fade={{ duration: 150 }}>
										<LoaderCircle class="size-7 animate-spin text-accent" />
									</div>
								{:else if item.state === "done"}
									<div
										class="absolute right-2 bottom-2 flex size-6 items-center justify-center rounded-full bg-success text-white shadow"
										in:scale={{ start: 0.3, duration: 350, easing: quintOut }}
									>
										<Check class="size-3.5" strokeWidth={3} />
									</div>
								{:else if item.state === "failed"}
									<div class="absolute inset-0 flex items-center justify-center bg-danger/20" in:fade>
										<X class="size-7 text-danger" />
									</div>
								{/if}

								{#if !busy}
									<button
										type="button"
										aria-label="Remover"
										onclick={() => items.splice(items.indexOf(item), 1)}
										class="absolute top-2 right-2 flex size-6 items-center justify-center rounded-full bg-black/55 text-white opacity-0 backdrop-blur-sm transition-opacity group-hover:opacity-100 hover:bg-black/75"
									>
										<X class="size-3.5" />
									</button>
								{/if}
							</div>
							<div class="px-2.5 py-2">
								<p class="truncate text-[13px]">{name(item.path)}</p>
								<p class="h-4 truncate text-xs {item.state === 'failed' ? 'text-danger' : 'text-text-3'}">
									{#if item.state === "done" && item.size !== undefined && item.inputSize}
										{kb(item.inputSize)} → {kb(item.size)}
										<span class={item.size < item.inputSize ? "text-success" : ""}>
											({item.size < item.inputSize ? "−" : "+"}{Math.abs(Math.round((1 - item.size / item.inputSize) * 100))}%)
										</span>
									{:else if item.state === "failed"}
										{item.error}
									{:else if item.state === "running"}
										Convertendo…
									{/if}
								</p>
							</div>
						</li>
					{/each}
				</ul>
			</div>
		{/if}

		{#if dragging}
			<div
				transition:fade={{ duration: 150 }}
				class="pointer-events-none absolute inset-0 bottom-4 left-4 flex items-center justify-center rounded-(--radius-card) border-2 border-accent bg-accent/10 backdrop-blur-md"
			>
				<div class="flex flex-col items-center gap-2 text-accent" in:scale={{ start: 0.9, duration: 250, easing: quintOut }}>
					<ImagePlus class="size-10" strokeWidth={1.5} />
					<p class="font-display text-lg font-semibold">Solte para adicionar</p>
				</div>
			</div>
		{/if}
	</section>

	<!-- Painel de opções -->
	<aside class="flex w-72 shrink-0 flex-col gap-5 overflow-y-auto px-4 pb-4">
		<div>
			<h2 class="mb-2 text-xs font-semibold text-text-2">Converter para</h2>
			<RadioGroup.Root bind:value={format} class="grid grid-cols-2 gap-1.5">
				{#each FORMATS as f (f.value)}
					<RadioGroup.Item
						value={f.value}
						class="flex flex-col items-start rounded-(--radius-control) border px-3 py-2 text-left transition-all duration-200 ease-(--ease-fluent) active:scale-[0.97]
							data-[state=checked]:border-accent data-[state=checked]:bg-accent/10
							data-[state=unchecked]:border-stroke data-[state=unchecked]:bg-card data-[state=unchecked]:hover:bg-card-hover"
					>
						{#snippet children({ checked })}
							<span class="text-sm font-semibold {checked ? 'text-accent' : ''}">{f.value.toUpperCase()}</span>
							<span class="text-[11px] text-text-3">{f.hint}</span>
						{/snippet}
					</RadioGroup.Item>
				{/each}
			</RadioGroup.Root>
		</div>

		{#if usesQuality}
			<div transition:fly={{ y: -6, duration: 200 }}>
				<div class="mb-3 flex items-baseline justify-between">
					<h2 class="text-xs font-semibold text-text-2">Qualidade</h2>
					<span class="text-sm font-semibold tabular-nums">{quality}</span>
				</div>
				<Slider.Root type="single" bind:value={quality} min={1} max={100} class="relative flex h-5 w-full touch-none items-center">
					<span class="relative h-1 w-full grow overflow-hidden rounded-full bg-stroke">
						<Slider.Range class="absolute h-full bg-accent" />
					</span>
					<Slider.Thumb
						index={0}
						class="block size-5 rounded-full border border-stroke shadow transition-transform duration-150 outline-none after:absolute after:inset-[5px] after:rounded-full after:bg-accent after:transition-transform hover:after:scale-125 focus-visible:ring-2 focus-visible:ring-accent active:after:scale-90 bg-white dark:bg-[#454545]"
					/>
				</Slider.Root>
				<div class="mt-1 flex justify-between text-[11px] text-text-3"><span>Menor arquivo</span><span>Melhor imagem</span></div>
			</div>
		{/if}

		<div>
			<h2 class="mb-2 text-xs font-semibold text-text-2">Salvar em</h2>
			<div class="flex flex-col gap-1.5">
				<button
					type="button"
					onclick={() => (outDir = null)}
					class="flex items-center gap-2 rounded-(--radius-control) border px-3 py-2 text-left transition-colors
						{outDir === null ? 'border-accent bg-accent/10' : 'border-stroke bg-card hover:bg-card-hover'}"
				>
					<span class="size-3.5 rounded-full border-2 {outDir === null ? 'border-[5px] border-accent' : 'border-text-3'} transition-all"></span>
					Mesma pasta do original
				</button>
				<button
					type="button"
					onclick={pickDir}
					class="flex min-w-0 items-center gap-2 rounded-(--radius-control) border px-3 py-2 text-left transition-colors
						{outDir !== null ? 'border-accent bg-accent/10' : 'border-stroke bg-card hover:bg-card-hover'}"
				>
					<FolderOpen class="size-4 shrink-0 {outDir !== null ? 'text-accent' : 'text-text-2'}" />
					<span class="min-w-0 truncate" title={outDir ?? ""}>{outDir ? name(outDir) : "Escolher pasta…"}</span>
				</button>
			</div>
		</div>

		<div class="mt-auto flex flex-col gap-2">
			{#if busy || done.length}
				<div class="h-1 overflow-hidden rounded-full bg-stroke" transition:fade>
					<div class="h-full rounded-full bg-accent transition-[width] duration-300 ease-(--ease-fluent)" style:width="{progress * 100}%"></div>
				</div>
			{/if}
			<button
				type="button"
				onclick={convert}
				disabled={busy || !pending}
				class="flex h-10 items-center justify-center gap-2 rounded-(--radius-control) bg-accent font-semibold text-on-accent shadow-sm transition-all duration-200 ease-(--ease-fluent) hover:bg-accent-hover active:scale-[0.98] disabled:opacity-40 disabled:active:scale-100"
			>
				{#if busy}
					<LoaderCircle class="size-4 animate-spin" /> Convertendo…
				{:else}
					Converter{pending ? ` ${pending}` : ""} para {format.toUpperCase()}
				{/if}
			</button>
		</div>
	</aside>
</div>
