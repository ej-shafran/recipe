<script lang="ts">
  import { onDestroy } from "svelte";
  import type { Action } from "svelte/action";

  import { getIn, setIn } from "./functions";
  import type { FormKey, FormResult, FormValues, ToErrors } from "./types";

  type Values = $$Generic<FormValues>;

  export let store: FormResult<Values>["store"];
  export let key: FormKey<Values>;
  export let label: string | undefined = undefined;

  const id = key.join("--");

  function updateTouched() {
    store.update((store) => ({
      ...store,
      touched: setIn(store.touched, key, true),
    }));
  }

  function updateValue(e: Event) {
    store.update((store) => {
      if (e.target && "value" in e.target) {
        return {
          ...store,
          values: setIn(store.values, key, e.target.value),
        };
      } else {
        return store;
      }
    });
  }

  // keep a local, up to date version of this field's
  // `touched`, `error`, and `value`
  let touched = getIn($store.touched, key) as boolean;
  let error = getIn(
    $store.errors,
    key as FormKey<ToErrors<Values>>
  ) as ToErrors<Values>;
  let value = getIn($store.values, key);
  const unsubscribe = store.subscribe((form) => {
    touched = getIn(form.touched, key) as boolean;
    error = getIn(
      form.errors,
      key as FormKey<ToErrors<Values>>
    ) as ToErrors<Values>;
    value = getIn(form.values, key);
  });
  onDestroy(unsubscribe);

  // keep a reference to the actual field's `HTMLElement`,
  // so we can update it every time `value` changes
  let nodeRef: HTMLElement | null = null;
  $: {
    if (nodeRef) {
      (nodeRef as any).value = value;
    }
  }

  // an action directive that manually sets the needed listeners,
  // because two-way-binding doesn't work through `let:` bindings
  const attributes: Action = (node) => {
    nodeRef = node;

    node.setAttribute("id", id);
    node.addEventListener("blur", updateTouched);
    node.addEventListener("input", updateValue);
    node.dataset.cy = key.join("_").toUpperCase();

    return {
      destroy() {
        node.removeAttribute("id");
        node.removeEventListener("blur", updateTouched);
        node.removeEventListener("input", updateValue);
        delete node.dataset.cy;
      },
    };
  };
</script>

<fieldset>
  <label for={id}>
    <slot name="label">
      <span>{label ?? ""}</span>
    </slot>
  </label>

  <slot {attributes}>
    <input use:attributes />
  </slot>

  <span data-cy={[...key, "error"].join("_").toUpperCase()}>
    {#if touched && error}
      {error._errors?.join(", ")}
    {/if}
  </span>
</fieldset>

<style>
  fieldset {
    border: none;
    display: flex;
    flex-direction: column;
    padding: 0;
  }

  span {
    min-height: 1.5rem;
  }
</style>
