<script lang="ts">
  import { createMutation, useQueryClient } from "@tanstack/svelte-query";
  import axios from "axios";

  export let id: string;

  let value = "";

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
    $mutation.mutate(value);
  }
</script>

<form action="#" on:submit|preventDefault={handleSubmit}>
  <input bind:value />

  <button type="submit">Submit</button>
</form>
