export type RecipePreviewDTO = {
  id: number;
  title: string;
  commentCount: number;
  poster: {
    username: string;
    id: string;
  };
};

