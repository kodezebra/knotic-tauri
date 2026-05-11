<script lang="ts">
	import favicon from '$lib/assets/favicon.svg';
  import '../app.css';
  import Layout from '$lib/components/Layout.svelte';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<Layout>
  {@render children()}
</Layout>
