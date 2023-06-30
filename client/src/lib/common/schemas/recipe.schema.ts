import { z } from "zod";

export const recipeSchema = z.object({
  title: z.string().min(10).max(180),
  content: z.string().min(20).max(3000),
});

export type RecipeValues = z.infer<typeof recipeSchema>;

export type RecipeFormErrors = Partial<z.ZodFormattedError<RecipeValues>>;
