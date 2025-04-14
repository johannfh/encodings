<script lang="ts">
	const apiUrl = 'http://localhost:8080';

	import { bytes, onlyUnique } from '@/lib/utils';
	import { onMount } from 'svelte';

	let dataToEncode = $state('');
	let encodeResult = $derived(bytes(dataToEncode));

	let connection: WebSocket | undefined = undefined;
	let messages: string[] = $state([]);
	let decoder = new TextDecoder();

	onMount(() => {
		connection = new WebSocket(apiUrl);

		connection.addEventListener('message', async (msg) => {
			console.log(msg);
			let data = msg.data as Blob;
			console.log(data);
			let result = await data.stream().getReader().read();
			let bytes = result.value;
			console.log(bytes);
			if (bytes) {
				let text = decoder.decode(bytes);
				messages.push(text);
			}
		});
	});
</script>

<div class="p-4">
	<form
		class="flex flex-row gap-4"
		onsubmit={(e) => {
			e.preventDefault();
			if (!connection) return;

			console.log('SENDING!');
			const message = encodeResult;
			dataToEncode = '';

			connection.send(message);
		}}
	>
		<input
			type="text"
			placeholder="Type in a message"
			class="grow rounded-xl bg-zinc-900 px-4 py-2"
			bind:value={dataToEncode}
		/>
		<button
			class="cursor-pointer rounded-xl bg-zinc-900 px-4 py-2 transition-all duration-300 hover:scale-110 hover:bg-zinc-900/70"
			onclick={() => {}}
			type="submit">Send</button
		>
	</form>
	<h2>Messages</h2>
	<div class="flex max-h-60 flex-col gap-4 overflow-y-auto font-mono">
		{#each messages.toReversed() as message}
			<div>
				{message}
			</div>
		{/each}
	</div>
	<div class="flex w-full flex-row py-2">
		<div class="w-1/2">
			<h2>Normal Encoded (UTF-8)</h2>
			<div>
				<h3>Character Bits</h3>
				<span class="grid grid-cols-[auto_auto_1fr] gap-x-2 font-mono">
					{#each [...new Intl.Segmenter().segment(dataToEncode)]
						.map((v) => v.segment)
						.toSorted()
						.filter(onlyUnique) as char}
						<span class="flex justify-center">"{char}"</span>
						<span>=</span>
						<span class="flex flex-row gap-x-2">
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
