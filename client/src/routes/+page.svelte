<script lang="ts">
	const routes = [
		"/browse",
		"/register",
		"/login",
		"/recipe/:id",
		"/user/:id/profile",
		"/user/:id"
	];
</script>

<h1>Recipe Website</h1>

{#each routes as route}
	<a href={route}>{route}</a>
  <br />
{/each}

