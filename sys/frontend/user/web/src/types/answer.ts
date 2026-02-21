import type { Challenge } from './challenge';
import type { UserSummary } from './user';

export interface Answer {
  id: string;
  challenge_id: string;
  user_id: string;
  content: string;
  score: number | null;
  ai_feedback: AiFeedback | null;
  like_count: number;
  comment_count: number;
  view_count: number;
  status: string;
  created_at: string;
  updated_at: string;
}

export interface AiFeedback {
  score: number;
  good_points: string;
  improvement: string;
  example_answer: string;
}

export interface AnswerWithUser extends Answer {
  user: UserSummary;
  is_liked: boolean;
}

export interface AnswerWithDetails extends Answer {
  user: UserSummary;
  challenge: Challenge;
  is_liked: boolean;
}

export interface CreateAnswerRequest {
  content: string;
}
