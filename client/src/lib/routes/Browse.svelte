<script lang="ts">
  import { createInfiniteQuery } from "@tanstack/svelte-query";
  import axios from "axios";

  import RecipePreview from "../components/RecipePreview.svelte";
  import Loading from "../components/Loading.svelte";
  import type { Paginated } from "../common/dto/Paginated.dto";
  import type { RecipePreviewDTO } from "../common/dto/RecipePreview.dto";
  import NewRecipe from "../components/NewRecipe.svelte";

  const limit = 2;

  const query = createInfiniteQuery({
    queryKey: ["browse-recipe-previews"],
    queryFn: async ({ pageParam: page = 1 }) => {
      const { data } = await axios<Paginated<RecipePreviewDTO>>({
        method: "GET",
        url: "/api/recipe/previews",
        params: {
          page,
          limit,
        },
      });

      return data;
    },
    getNextPageParam: (last) => last.nextPage ?? undefined,
  });
</script>

<h1>Browse Page</h1>

{#if $query.isLoading}
  <Loading />
{:else if $query.isError}
  <div>
    ERROR! <pre>{JSON.stringify($query.error)}</pre>
  </div>
{:else}
  <ul>
    {#each $query.data.pages.flatMap((page) => page.results) as recipePreview}
      <li>
        <RecipePreview {recipePreview} />
      </li>
    {/each}
  </ul>
{/if}

{#if $query.hasNextPage}
  <button on:click={() => $query.fetchNextPage()}>Next Page</button>
{/if}

<br />

<NewRecipe />
