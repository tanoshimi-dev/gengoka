export interface Category {
  id: string;
  name: string;
  description: string | null;
  icon: string | null;
  color: string | null;
  char_limit: number;
  sort_order: number;
  status: string;
  created_at: string;
  updated_at: string;
}

export interface CategoryResponse {
  id: string;
  name: string;
  description: string | null;
  icon_name: string | null;
  color_hex: string | null;
  challenge_count: number;
}
