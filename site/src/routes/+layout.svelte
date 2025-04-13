<script lang="ts">
	import * as Nav from '@/components/navigation';
	import '../app.css';

	let { children } = $props();
</script>

<div class="flex flex-col">
	<div class="sticky top-0 h-16 w-full">
		<Nav.Group>
			<h1 class="transition-all duration-300 hover:opacity-80">
				<a href="/">Encodings</a>
			</h1>
			<Nav.Link title="Huffman Encoding" href="/huffman" />
		</Nav.Group>
	</div>

	<div class="grow overflow-y-auto">
		{@render children()}
	</div>
</div>
