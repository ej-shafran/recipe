<script lang="ts">
  import type { Comment } from "../common/dto/Comment.dto";

  export let comment: Comment;
</script>

<hr />

<div data-cy="RECIPE_COMMENT">
  <h4>{comment.poster.username}</h4>
  <p>{comment.content}</p>
</div>
