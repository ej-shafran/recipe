<script lang="ts">
  import { createInfiniteQuery } from "@tanstack/svelte-query";
  import axios from "axios";

  import type { Paginated } from "../common/dto/Paginated.dto";
  import type { Comment } from "../common/dto/Comment.dto";
  import CommentPreview from "./CommentPreview.svelte";
  import NewComment from "./NewComment.svelte";
  import Loading from "./Loading.svelte";

  export let id: string;

  const limit = 10;

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
    getNextPageParam: (last) => last.nextPage ?? undefined,
  });

  let bottomOfList: HTMLDivElement | null = null;
  const observer = new IntersectionObserver(
    ([target]) => {
      if (target.isIntersecting && $query.hasNextPage) $query.fetchNextPage();
    },
    {
      root: null,
      rootMargin: "200px",
    }
  );

  $: if (bottomOfList) observer.observe(bottomOfList);
</script>

{#if $query.isLoading}
  <Loading />
{:else if $query.isError}
  <div>
    ERROR! <pre>{JSON.stringify($query.error)}</pre>
  </div>
{:else}
  <h3>Comments</h3>

  <NewComment {id} />

  {#each $query.data.pages.flatMap((page) => page.results) as comment}
    <CommentPreview {comment} />
  {/each}

  <div bind:this={bottomOfList} />
{/if}
