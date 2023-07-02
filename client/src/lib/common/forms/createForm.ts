import { writable } from "svelte/store";
import type {
  CreateMutateFunction,
  CreateMutationResult,
} from "@tanstack/svelte-query";

import type { FormResult, FormSchema, ToErrors } from "./types";
import { zTouched, zValues } from "./functions";

export function createForm<Values extends Record<PropertyKey, any>>(
  schema: FormSchema<Values>,
  mutation: CreateMutationResult<any, any, Values>
): FormResult<Values> {
  const initialValues = zValues(schema);
  const initialTouched = zTouched(schema);
  const initialErrors = {} as ToErrors<Values>;

  let values = initialValues;
  let touched = initialTouched;
  let errors = initialErrors;

  let mutate: CreateMutateFunction<any, any, Values> | null = null;
  const mutationUnusb = mutation.subscribe((base) => (mutate = base.mutate));

  const store = writable({ values, touched, errors });

  const storeUnsub = store.subscribe((store) => {
    values = store.values;
    touched = store.touched;
    errors = store.errors;
  });

  function validate(values: unknown) {
    const validation = schema.safeParse(values);

    if (!validation.success) {
      store.update((store) => ({
        ...store,
        errors: validation.error.format() as ToErrors<Values>,
      }));
    } else {
      store.update((store) => ({ ...store, errors: {} as ToErrors<Values> }));
    }

    return validation.success;
  }

  function handleSubmit() {
    store.update((store) => ({ ...store, touched: zTouched(schema, true) }));

    if (!validate(values)) return;

    if (mutate) mutate(values);
  }

  function reset() {
    store.set({
      values: initialValues,
      touched: initialTouched,
      errors: initialErrors,
    });
  }

  return {
    store,
    validate,
    handleSubmit,
    unsubscribe: () => {
      mutationUnusb();
      storeUnsub();
    },
    reset,
  };
}
