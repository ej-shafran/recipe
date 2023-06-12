export type RecipeDetailsDTO = {
  id: number;
  title: string;
  content: string;
  poster: {
    username: string;
    id: string;
  };
};
