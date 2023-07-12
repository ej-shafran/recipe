<script lang="ts">
  import { createMutation, useQueryClient } from "@tanstack/svelte-query";
  import axios from "axios";
  import { z } from "zod";

  import { createForm } from "../common/forms/createForm";
  import Form from "../common/forms/Form.svelte";
  import Field from "../common/forms/Field.svelte";
  import SubmitButton from "../common/forms/SubmitButton.svelte";
  import { schemas } from "../common/forms/schemas";

  export let id: string;

  const queryClient = useQueryClient();

  const mutation = createMutation({
    mutationKey: ["new-comment", id],
    mutationFn: async (values: z.infer<typeof schemas.comment>) => {
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

  const form = createForm(schemas.comment, mutation);
  const { store, reset } = form;
</script>

<Form {...form}>
  <Field {store} key={["content"]} />

  <SubmitButton>Comment</SubmitButton>
</Form>
