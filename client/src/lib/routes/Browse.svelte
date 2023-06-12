<script lang="ts">
  import { createInfiniteQuery } from "@tanstack/svelte-query";
  import axios from "axios";

  import type { PreviewData } from "../common/dto/RecipePreview.dto";
  import RecipePreview from "../components/RecipePreview.svelte";

  const LIMIT = 2;

  const query = createInfiniteQuery({
    queryKey: ["browse-recipe-previews"],
    queryFn: async ({ pageParam = 1 }) => {
      const { data } = await axios.get<PreviewData>("/api/recipes/previews", {
        params: {
          page: pageParam,
          limit: LIMIT,
        },
      });

      return data;
    },
    getNextPageParam: (last) => last.nextPage ?? undefined,
  });
</script>

<h1>Browse Page</h1>

{#if $query.isLoading}
  <div>Loading...</div>
{:else if $query.isError}
  <div>
    ERROR! <pre>{JSON.stringify($query.error)}</pre>
  </div>
{:else}
  <div>
    {#each $query.data.pages.flatMap((page) => page.results) as recipePreview}
      <RecipePreview {recipePreview} />
    {/each}
  </div>
{/if}

{#if $query.hasNextPage}
  <button on:click={() => $query.fetchNextPage()}>Next Page</button>
{/if}
