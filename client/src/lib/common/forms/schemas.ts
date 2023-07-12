import { z } from "zod";
import { errors } from "./errors";

export namespace schemas {
  export const username = z
    .string()
    .min(5, errors.min(5))
    .max(30, errors.max(30));

  export const password = z
    .string()
    .min(5, errors.min(5))
    .max(100, errors.max(100));

  export const credentials = z.object({
    username,
    password,
  });

  export const register = z
    .object({
      confirmPassword: password,
      ...credentials.shape,
    })
    .refine((schema) => schema.password === schema.confirmPassword, {
      message: errors.confirm(),
      path: ["confirmPassword"],
    });

  export const comment = z.object({
    content: z.string().min(10).max(255),
  });

  export const recipe = z.object({
    title: z.string().min(10).max(180),
    content: z.string().min(20).max(3000),
  });
}
