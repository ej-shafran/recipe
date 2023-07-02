<script lang="ts">
  import { createMutation, useQueryClient } from "@tanstack/svelte-query";
  import axios from "axios";
  import { z } from "zod";

  import { createForm } from "../common/forms/createForm";
  import Form from "../common/forms/Form.svelte";
  import Field from "../common/forms/Field.svelte";

  export let id: string;

  const schema = z.object({
    content: z.string().min(10).max(255),
  });

  const queryClient = useQueryClient();

  const mutation = createMutation({
    mutationKey: ["new-comment", id],
    mutationFn: async (values: z.infer<typeof schema>) => {
      await axios({
        method: "POST",
        url: `/api/comment/${id}`,
        data: values,
      });
    },
    onSuccess: () => {
      queryClient.invalidateQueries(["comments", id]);
    },
    onSettled: () => {
      reset();
    },
  });

  const form = createForm(schema, mutation);
  const { store, reset } = form;
</script>

<Form {...form}>
  <Field {store} key={["content"]} />

  <button type="submit">Comment</button>
</Form>
