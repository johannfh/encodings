<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		children?: Snippet<[]>;
	}

	const { children }: Props = $props();
</script>

<div
	class="flex size-full flex-row items-center gap-4 bg-indigo-700/10 px-4 py-2 backdrop-blur-xl backdrop-opacity-90"
>
	{#if children}
		{@render children()}
	{/if}
</div>
