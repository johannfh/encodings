<script lang="ts">
	import { encodeData } from '@/lib/huffman';
	import { bytes } from '@/lib/utils';
	import { onMount } from 'svelte';

	let dataToEncode = 'aaabbc💀';
	let encodeResult = new Uint8Array([1, 2, 1, 5, 0]);

	onMount(() => bytes(dataToEncode));
</script>

<div class="p-4">
	<input type="text" placeholder="Data to encode" class="p-2" bind:value={dataToEncode} />
	<button
		class="cursor-pointer rounded-lg bg-zinc-700 p-2 transition-all hover:scale-120 hover:bg-zinc-600"
		type="button"
		onclick={() => bytes(dataToEncode.toWellFormed())}>Encode</button
	>
	<div>
		<span>Result:</span>
		<span>{encodeResult}</span>
	</div>
</div>

<style>
</style>
