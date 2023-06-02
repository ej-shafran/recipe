<script lang="ts">
  import type { NavigateFn } from "svelte-navigator";

  export let navigate: NavigateFn;
  navigate("/login", { replace: true });

  // ignoring `svelte-navigator` props
  export let location: unknown;
  location;
</script>
