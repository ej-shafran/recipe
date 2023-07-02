<script lang="ts">
  import { z } from "zod";
  import { createMutation } from "@tanstack/svelte-query";
  import axios from "axios";
  import { navigate } from "svelte-navigator";

  import { createForm } from "../common/forms/createForm";
  import Form from "../common/forms/Form.svelte";
  import Field from "../common/forms/Field.svelte";

  export const schema = z.object({
    title: z.string().min(10).max(180),
    content: z.string().min(20).max(3000),
  });

  const mutation = createMutation({
    mutationKey: ["new-recipe"],
    mutationFn: async (recipe: z.infer<typeof schema>) => {
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

  const form = createForm(schema, mutation);
  const { store } = form;
</script>

<Form {...form}>
  <Field {store} key={["title"]} label="Title:" />
  <Field {store} key={["content"]} let:attributes label="Content:">
    <textarea use:attributes />
  </Field>

  <button type="submit">Submit</button>
</Form>
