<script lang="ts">
  import { onDestroy } from "svelte";
  import type { FormResult } from "./types";

  type Values = $$Generic<Record<PropertyKey, any>>;

  interface $$Props extends FormResult<Values> {}

  export let handleSubmit: FormResult<Values>["handleSubmit"];
  export let validate: FormResult<Values>["validate"];
  export let store: FormResult<Values>["store"];
  export let unsubscribe: FormResult<Values>["unsubscribe"];
  export let reset: FormResult<Values>["reset"];

  // IGNORE-PROP
  reset;

  onDestroy(unsubscribe);

  $: validate($store.values);
</script>

<form on:submit|preventDefault={handleSubmit}>
  <slot />
</form>

<style>
  form {
    width: 500px;
  }
</style>
