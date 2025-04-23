<script lang="ts">
	import * as d3 from 'd3';
	import { onMount } from 'svelte';

	interface Node {
		frequency?: number;
		left?: Node;
		right?: Node;
		byte?: number;
		prefix?: string;
	}

	interface Props {
		width: number;
		height: number;
		treeRoot?: Node;
	}

	let { width, height, treeRoot }: Props = $props();

	onMount(() => {
		if (!svgRef) return;

		const svg = d3.select<SVGSVGElement, Node>(svgRef);
		svg.selectAll('*').remove();

		if (!treeRoot) return;

		const x = svg.data();

		const hierarchyRoot = d3
			.hierarchy(treeRoot, (d) => [d.left, d.right].filter((e) => e !== undefined))
			.each((d) => {
				if (d.parent) {
					d.data.prefix = d.parent.children!.indexOf(d).toString();
				} else {
					d.data.prefix = '';
				}
			});

		const treeLayout = d3
			.tree<Node>()
			.separation((a, b) => (a.parent == b.parent ? 1 : 2))
			.size([width, height]);

		treeLayout(hierarchyRoot);
	});

	let svgRef: SVGSVGElement | undefined;
</script>

<svg bind:this={svgRef} {width} {height} class="border border-black" />
