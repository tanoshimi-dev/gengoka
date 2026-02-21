export interface FeedQueryParams {
  page?: number;
  page_size?: number;
  filter?: 'all' | 'following' | string;
  category_id?: string;
}

export interface PaginationParams {
  page?: number;
  page_size?: number;
}

export interface AnswerQueryParams {
  page?: number;
  page_size?: number;
  sort?: 'latest' | 'popular' | 'trending';
}
