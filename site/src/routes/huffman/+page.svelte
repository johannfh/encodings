<script lang="ts">
	import { bytes, onlyUnique } from '@/lib/utils';

	let dataToEncode = $state('aaabbc💀');

	let encodeResult = $derived(bytes(dataToEncode));

	$effect(() => {
		let splitted = [...new Intl.Segmenter().segment(dataToEncode)].map((v) => v.segment);
		console.log(splitted);
	});

	console.log(['a', 'a', 'b', 'c']);
</script>

<div class="p-4">
	<input
		type="text"
		placeholder="Data to encode"
		class="w-full rounded-xl bg-zinc-900 p-4"
		bind:value={dataToEncode}
	/>
	<div class="flex w-full flex-row py-2">
		<div class="w-1/2">
			<h2>Normal Encoded (UTF-8)</h2>
			<div>
				<h3>Character Bits</h3>
				<span class="grid grid-cols-[auto_auto_1fr] gap-x-2">
					{#each [...new Intl.Segmenter().segment(dataToEncode)]
						.map((v) => v.segment)
						.toSorted()
						.filter(onlyUnique) as char}
						<span class="flex justify-center">{char}</span>
						<span>=</span>
						<span class="flex flex-row gap-x-2 font-mono">
							{#each bytes(char) as byte}
								<span>{byte.toString(2).padStart(8, '0')}</span>
							{/each}
						</span>
					{/each}
				</span>
				<h3>Result</h3>
				<div class="font-bold">Size: {encodeResult.length * 8} bits</div>
				<span class="flex flex-wrap gap-2 font-mono">
					{#each encodeResult as byte}
						<span>{byte.toString(2).padStart(8, '0')}</span>
					{/each}
				</span>
			</div>
		</div>
		<div class="w-1/2">
			<h2>Huffman Encoded</h2>
			<span></span>
		</div>
	</div>
</div>

<style>
</style>
