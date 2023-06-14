export type Paginated<T> = {
  results: T[];
  nextPage: number | null;
}
