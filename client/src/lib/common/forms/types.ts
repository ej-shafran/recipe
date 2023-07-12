import type { CreateBaseMutationResult } from "@tanstack/svelte-query";
import type { Unsubscriber, Writable } from "svelte/store";
import type { z } from "zod";

export type FormValues = Record<PropertyKey, any>;

export type FormSchema<Values extends FormValues> =
  | z.ZodEffects<z.ZodObject<any, any, any, Values>>
  | z.ZodObject<any, any, any, Values>;
export type FormMutation<Values extends FormValues> =
  CreateBaseMutationResult<Values>;

export type ToErrors<Values extends FormValues> = {
  _errors?: string[];
} & {
    [P in keyof Values]?: Values[P] extends FormValues
    ? { _errors?: string[] } & ToErrors<Values[P]>
    : { _errors?: string[] };
  };

export type ToTouched<Values extends FormValues> = {
  [P in keyof Values]: Values[P] extends FormValues
  ? ToTouched<Values>
  : boolean;
};

export type ValidateFunction = (values: unknown) => boolean;
export type SubmitHandler = () => void;

export type FormKey<Values extends FormValues> = {
  [P1 in keyof Values]: Values[P1] extends FormValues
  ?
  | [P1]
  | {
    [P2 in keyof Values[P1]]: Values[P1][P2] extends FormValues
    ?
    | [P1, P2]
    | {
      [P3 in keyof Values[P1][P2]]: Values[P1][P2] extends FormValues
      ?
      | [P1, P2, P3]
      | {
        [P4 in keyof Values[P1][P2][P3]]: Values[P1][P2][P3][P4] extends FormValues
        ? [P1, P2, P3, P4, ...PropertyKey[]]
        : [P1, P2, P3, P4];
      }[keyof Values[P1][P2][P3]]
      : [P1, P2, P3];
    }[keyof Values[P1][P2]]
    : [P1, P2];
  }[keyof Values[P1]]
  : [P1];
}[keyof Values];

export type FormResult<Values extends FormValues> = {
  store: Writable<{
    values: Values;
    touched: ToTouched<Values>;
    errors: ToErrors<Values>;
  }>;
  validate: ValidateFunction;
  handleSubmit: SubmitHandler;
  unsubscribe: Unsubscriber;
  reset: () => void;
};
