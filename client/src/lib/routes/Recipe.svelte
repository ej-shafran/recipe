<script lang="ts">
  import { createQuery } from "@tanstack/svelte-query";
  import axios from "axios";
  import { Link } from "svelte-navigator";

  import Loading from "../components/Loading.svelte";

  export let id: string;

  const query = createQuery({
    queryKey: ["recipe-details", id],
    queryFn: async () => {
      const { data } = await axios({
        method: "GET",
        url: `/api/recipes/${id}`,
      });

      return data;
    },
  });
</script>

<h1>Recipe Page</h1>

<h2>ID: {id}</h2>

{#if $query.isLoading}
  <Loading />
{:else if $query.isError}
  <div>
    ERROR! <pre>{JSON.stringify($query.error)}</pre>
  </div>
{:else}
  <div>
    DATA! <pre>{JSON.stringify($query.data, null, 2)}</pre>
  </div>
{/if}

<Link to="/browse">Back to Browse</Link>
