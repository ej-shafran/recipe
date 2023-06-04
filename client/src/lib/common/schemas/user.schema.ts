import { z } from "zod";

export const userSchema = z.object({
  username: z.string().min(5).max(60),
  password: z.string().min(5).max(60),
});

export type UserValues = z.infer<typeof userSchema>;

export type UserFormErrors = z.ZodFormattedError<UserValues>;
