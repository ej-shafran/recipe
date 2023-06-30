<script lang="ts">
  import { createInfiniteQuery } from "@tanstack/svelte-query";
  import axios from "axios";
  import type { Paginated } from "../common/dto/Paginated.dto";
  import type { Comment } from "../common/dto/Comment.dto";
  import CommentPreview from "./CommentPreview.svelte";

  export let id: string;

  const limit = 5;

  const query = createInfiniteQuery({
    queryKey: ["comments", id],
    queryFn: async ({ pageParam: page = 1 }) => {
      const { data } = await axios<Paginated<Comment>>({
        method: "GET",
        url: `/api/comment/${id}`,
        params: {
          page,
          limit,
        },
      });

      return data;
    },
    getNextPageParam: (lastPage) => lastPage.nextPage ?? undefined,
  });
</script>

{#if $query.isLoading}
  <h3>Loading comments...</h3>
{:else if $query.isError}
  <h3>Recipe Comments: Error</h3>
  <div>
    ERROR! <pre>{JSON.stringify($query.error)}</pre>
  </div>
{:else}
  <h3>Comments</h3>
  {#each $query.data.pages.flatMap((page) => page.results) as comment}
    <CommentPreview {comment} />
  {/each}
{/if}

{#if $query.hasNextPage}
  <button on:click={() => $query.fetchNextPage()}>Load More</button>
{/if}
