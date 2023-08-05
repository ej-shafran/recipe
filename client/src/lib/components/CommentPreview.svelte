<script lang="ts">
  import { createMutation, useQueryClient } from "@tanstack/svelte-query";
  import type { Comment } from "../common/dto/Comment.dto";
  import { getUserId } from "../common/functions/auth.functions";
  import axios from "axios";

  export let comment: Comment;
  export let recipeId: string;

  const userId = getUserId();
  const queryClient = useQueryClient();

  const mutation = createMutation({
    mutationFn: async () => {
      await axios.delete(`/api/comment/${comment.id}`);
    },
    onSuccess: () => {
      queryClient.invalidateQueries(["comments", recipeId]);
    },
  });
</script>

<hr />

{#if $mutation.isLoading}
  <!-- TODO -->
  <div>Loading...</div>
{:else}
  <div data-cy="RECIPE_COMMENT">
    <h4>{comment.poster.username}</h4>
    <p>{comment.content}</p>

    {#if comment.poster.id === userId}
      <button data-cy="DELETE_COMMENT" on:click={() => $mutation.mutate()}>Delete</button>
    {/if}
  </div>
{/if}
