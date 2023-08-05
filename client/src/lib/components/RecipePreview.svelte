<script lang="ts">
  import { Link } from "svelte-navigator";
  import axios from "axios";
  import { createMutation, useQueryClient } from "@tanstack/svelte-query";

  import type { RecipePreviewDTO } from "../common/dto/RecipePreview.dto";
  import { getUserId } from "../common/functions/auth.functions";

  export let recipePreview: RecipePreviewDTO;

  const userId = getUserId();
  const queryClient = useQueryClient();

  const mutation = createMutation({
    mutationFn: async () => {
      await axios.delete(`/api/recipe/${recipePreview.id}`);
    },
    onSuccess: () => {
      queryClient.invalidateQueries(["browse-recipe-previews"]);
    },
  });
</script>

{#if $mutation.isLoading}
  <!-- TODO -->
  <div>Loading...</div>
{:else}
  <div>
    <h3>
      <Link to="/recipe/{recipePreview.id}">
        {recipePreview.title}
      </Link>
    </h3>

    <p>Posted By: <span>{recipePreview.poster.username}</span></p>
    <p>Comments: <span>{recipePreview.commentCount}</span></p>

    {#if recipePreview.poster.id === userId}
      <button data-cy="DELETE_RECIPE" on:click={() => $mutation.mutate()}>Delete</button>
    {/if}
  </div>
{/if}
