<script lang="ts">
  import type { RecipeDetailsDTO } from "../common/dto/RecipeDetails.dto";

  export let recipeDetails: RecipeDetailsDTO;
</script>

<div>
  <h1 data-cy="RECIPE_TITLE">{recipeDetails.title}</h1>
  <h2>by <span>{recipeDetails.poster.username}</span></h2>

  <p>{recipeDetails.content}</p>
</div>
