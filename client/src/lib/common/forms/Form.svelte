<script lang="ts">
  import { onDestroy } from "svelte";
  import type { FormResult } from "./types";

  type Values = $$Generic<Record<PropertyKey, any>>;

  interface $$Props extends FormResult<Values> {
    "data-cy"?: string;
  }

  export let handleSubmit: FormResult<Values>["handleSubmit"];
  export let validate: FormResult<Values>["validate"];
  export let store: FormResult<Values>["store"];
  export let unsubscribe: FormResult<Values>["unsubscribe"];
  export let reset: FormResult<Values>["reset"];

  let prevValues: Values | null = null;

  // IGNORE-PROP
  reset;

  onDestroy(unsubscribe);

  $: if ($store.values !== prevValues) {
    validate($store.values);
    prevValues = $store.values;
  }
</script>

<form on:submit|preventDefault={handleSubmit} data-cy={$$restProps["data-cy"]}>
  <slot />
</form>

<style>
  form {
    width: 500px;
  }
</style>
