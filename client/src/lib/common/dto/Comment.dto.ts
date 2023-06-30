export type Comment = {
  id: number;
  content: string;
  poster: {
    id: string;
    username: string;
  };
};
