<script lang="ts">
	import * as Nav from '@/components/navigation';
	import '../app.css';

	let { children } = $props();
</script>

<Nav.Group>
	<Nav.Heading title="Encodings" href="/" />
	<Nav.Link title="Huffman Encoding" href="/huffman" />
</Nav.Group>

<div>
	{@render children()}
</div>
