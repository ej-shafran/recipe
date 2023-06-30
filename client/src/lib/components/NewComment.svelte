<script lang="ts">
  import { createMutation, useQueryClient } from "@tanstack/svelte-query";
  import axios from "axios";
  import { z } from "zod";

  export let id: string;

  const schema = z.string().min(10).max(255);

  let value = "";
  let error: string | null = null;

  const queryClient = useQueryClient();

  const mutation = createMutation({
    mutationKey: ["new-comment", id],
    mutationFn: async (content: string) => {
      await axios({
        method: "POST",
        url: `/api/comment/${id}`,
        data: {
          content,
        },
      });
    },
    onSuccess: () => {
      queryClient.invalidateQueries(["comments", id]);
    },
    onSettled: () => {
      value = "";
    },
  });

  function handleSubmit() {
    const validation = schema.safeParse(value);

    if (!validation.success) {
      error = validation.error.format()._errors.join(", ");
    } else {
      error = null;
      $mutation.mutate(value);
    }
  }
</script>

<form action="#" on:submit|preventDefault={handleSubmit}>
  <input bind:value />

  {#if !!error}
    <p>{error}</p>
  {/if}

  <button type="submit">Submit</button>
</form>
