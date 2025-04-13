<script lang="ts">
	interface Props {
		href: string;
		title: string;
	}

	const { href, title }: Props = $props();
</script>

<a class="cursor-pointer transition-all duration-300 hover:scale-110 hover:opacity-80" {href}
	>{title}</a
>
