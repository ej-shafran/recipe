<script lang="ts">
  import { Link } from "svelte-navigator";
  import type { RecipePreviewDTO } from "../common/dto/RecipePreview.dto";

  export let recipePreview: RecipePreviewDTO;
</script>

<div>
  <h3>
    <Link to="/recipe/{recipePreview.id}">
      {recipePreview.title}
    </Link>
  </h3>

  <p>Posted By: <span>{recipePreview.poster.username}</span></p>
  <p>Comments: <span>{recipePreview.commentCount}</span></p>
</div>
