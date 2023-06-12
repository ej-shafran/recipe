<script lang="ts">
  import { createQuery } from "@tanstack/svelte-query";
  import axios from "axios";
  import { Link } from "svelte-navigator";

  import Loading from "../components/Loading.svelte";
  import RecipeDetails from "../components/RecipeDetails.svelte";
  import type { RecipeDetailsDTO } from "../common/dto/RecipeDetails.dto";

  export let id: string;

  const query = createQuery({
    queryKey: ["recipe-details", id],
    queryFn: async () => {
      const { data } = await axios<RecipeDetailsDTO>({
        method: "GET",
        url: `/api/recipes/${id}`,
      });

      return data;
    },
  });
</script>

{#if $query.isLoading}
  <h1>Recipe Page: Loading...</h1>
  <Loading />
{:else if $query.isError}
  <h1>Recipe Page: Error</h1>
  <div>
    ERROR! <pre>{JSON.stringify($query.error)}</pre>
  </div>
{:else}
  <RecipeDetails recipeDetails={$query.data} />
{/if}

<Link to="/browse">Back to Browse</Link>
