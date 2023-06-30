<script lang="ts">
  import type { Comment } from "../common/dto/Comment.dto";

  export let comment: Comment;
</script>

<hr />

<div>
  <h4>{comment.poster.username}</h4>
  <p>{comment.content}</p>
</div>
