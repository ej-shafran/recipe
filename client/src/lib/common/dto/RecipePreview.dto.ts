export type RecipePreviewDTO = {
  id: number;
  title: string;
  commentCount: number;
  poster: {
    username: string;
    id: string;
  };
};

export type PreviewData = {
  nextPage: number | null;
  results: RecipePreviewDTO[];
};
