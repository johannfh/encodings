<script lang="ts">
	interface Props {
		href: string;
		title: string;
	}

	const { href, title }: Props = $props();
</script>

<a class="link" {href}>{title}</a>

<style>
	.link {
	}
</style>
