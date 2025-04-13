<script lang="ts">
	import type { Snippet, SvelteComponent } from 'svelte';
	import type { SvelteHTMLElements } from 'svelte/elements';

	interface Props {
		children?: Snippet<[]>;
	}

	const { children }: Props = $props();
</script>

<div class="flex flex-col gap-4 px-4 py-2">
	{#if children}
		{@render children()}
	{/if}
</div>
