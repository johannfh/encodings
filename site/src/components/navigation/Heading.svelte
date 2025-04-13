<script lang="ts">
	interface Props {
		headingType: HeadingType;
		title: string;
		href?: string;
	}
	type HeadingType = 'h1' | 'h2' | 'h3';

	let { headingType, title, href }: Props = $props();
</script>

{#snippet inner(title: string)}
	{#if headingType === 'h1'}
		<h1>{title}</h1>
	{:else if headingType === 'h2'}
		<h2>{title}</h2>
	{:else}
		<h3>{title}</h3>
	{/if}
{/snippet}

{#if href}
	<a {href}>
		{@render inner(title)}
	</a>
{:else}
	{@render inner(title)}
{/if}

<style>
	h1,
	h2,
	h3 {
		@apply flex underline underline-offset-2;
	}
</style>
