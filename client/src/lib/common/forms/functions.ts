import { z } from "zod";
import type { FormKey, FormSchema, FormValues, ToTouched } from "./types";

export function zShape<Values extends FormValues>(schema: FormSchema<Values>) {
  return "shape" in schema ? schema.shape : schema.innerType().shape;
}

export function zValues<Values extends FormValues>(
  schema: FormSchema<Values>
): Values {
  return Object.fromEntries(
    Object.entries(zShape(schema)).map(([key, value]) => {
      if (value instanceof z.ZodDefault)
        return [key, value._def.defaultValue()];
      if (value instanceof z.ZodObject) return [key, zValues(value)];
      if (value instanceof z.ZodArray) return [key, []];
      if (value instanceof z.ZodString) return [key, ""];
      if (value instanceof z.ZodNumber) return [key, 0];
      if (value instanceof z.ZodBoolean) return [key, false];
      if (value instanceof z.ZodDate) return [key, new Date()];
      return [key, undefined];
    })
  ) as Values;
}

export function zTouched<Values extends FormValues>(
  schema: FormSchema<Values>,
  touch = false
): ToTouched<Values> {
  return Object.fromEntries(
    Object.entries(zShape(schema)).map(([key, value]) => {
      if (value instanceof z.ZodObject) return [key, zTouched(value, touch)];
      return [key, touch];
    })
  ) as ToTouched<Values>;
}

export function setIn<T extends FormValues>(
  obj: T,
  pathArray: FormKey<T>,
  value: any
): any {
  let res = JSON.parse(JSON.stringify(obj));
  let resVal = res;
  let i = 0;

  for (; i < pathArray.length - 1; i++) {
    const currentPath = pathArray[i];
    let currentObj: any = getIn(obj, pathArray.slice(0, i + 1) as FormKey<T>);

    if (currentObj && typeof currentObj === "object") {
      resVal = resVal[currentPath] = JSON.parse(JSON.stringify(currentObj));
    } else {
      const nextPath = pathArray[i + 1];
      resVal = resVal[currentPath] =
        !isNaN(Number(nextPath)) &&
          Number(nextPath) === Math.floor(Number(nextPath)) &&
          Number(nextPath) >= 0
          ? []
          : {};
    }
  }

  // Return original object if new value is the same as current
  if ((i === 0 ? obj : resVal)[pathArray[i]] === value) {
    return obj;
  }

  if (value === undefined) {
    delete resVal[pathArray[i]];
  } else {
    resVal[pathArray[i]] = value;
  }

  // If the path array has a single element, the loop did not run.
  // Deleting on `resVal` had no effect in this scenario, so we delete on the result instead.
  if (i === 0 && value === undefined) {
    delete res[pathArray[i]];
  }

  return res;
}

export function getIn<TObj extends FormValues, TResult = any>(
  obj: TObj,
  pathArray: FormKey<TObj>
): TResult {
  let result = obj;
  for (const key of pathArray) {
    if (!result) break;
    result = result[key as any];
  }
  return result;
}
