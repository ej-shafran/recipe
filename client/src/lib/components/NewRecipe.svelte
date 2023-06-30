<script lang="ts">
  import { createMutation } from "@tanstack/svelte-query";
  import { recipeSchema } from "../common/schemas/recipe.schema";
  import type {
    RecipeFormErrors,
    RecipeValues,
  } from "../common/schemas/recipe.schema";
  import axios from "axios";
  import { navigate } from "svelte-navigator";

  const touched = {
    title: false,
    content: false,
  };

  let title = "";
  let content = "";

  let errors: RecipeFormErrors = {};

  const mutation = createMutation({
    mutationKey: ["new-recipe"],
    mutationFn: async (recipe: RecipeValues) => {
      const { data } = await axios<number>({
        method: "POST",
        url: "/api/recipe",
        data: recipe,
      });

      return data;
    },
    onSuccess: (id) => {
      navigate(`/recipe/${id}`);
    },
  });

  function handleSubmit() {
    if (!validate(title, content)) return;

    $mutation.mutate({ title, content });
  }

  function touch(key: keyof RecipeValues) {
    if (!touched[key]) touched[key] = true;
  }

  function validate(title: string, content: string) {
    const validation = recipeSchema.safeParse({ title, content });

    if (!validation.success) {
      errors = validation.error.format();
    } else {
      errors = {};
    }

    return validation.success;
  }

  $: validate(title, content);
</script>

<form on:submit|preventDefault={handleSubmit}>
  <div>
    <label for="title"> Title: </label>
    <input
      id="title"
      type="text"
      bind:value={title}
      on:blur={() => touch("title")}
    />
    <span>
      {#if touched.title}
        {errors?.title?._errors ?? ""}
      {/if}
    </span>
  </div>

  <div>
    <label for="content"> Content: </label>
    <textarea
      id="content"
      bind:value={content}
      on:blur={() => touch("content")}
    />
    <span>
      {#if touched.content}
        {errors?.content?._errors ?? ""}
      {/if}
    </span>
  </div>

  <button type="submit">Submit</button>
</form>
