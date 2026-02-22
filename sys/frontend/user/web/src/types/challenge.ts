import type { Answer } from './answer';
import type { Category } from './category';

export interface Challenge {
  id: string;
  category_id: string;
  title: string;
  description: string | null;
  char_limit: number;
  release_date: string | null;
  answer_count: number;
  status: string;
  created_at: string;
  updated_at: string;
}

export interface ChallengeWithCategory extends Challenge {
  category: Category;
}

export interface DailyChallengeResponse {
  challenge: Challenge;
  category_name: string;
  is_completed: boolean;
  user_answer: Answer | null;
}
