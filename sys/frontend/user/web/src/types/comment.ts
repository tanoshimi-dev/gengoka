import type { UserSummary } from './user';

export interface Comment {
  id: string;
  answer_id: string;
  user_id: string;
  content: string;
  status: string;
  created_at: string;
  updated_at: string;
}

export interface CommentWithUser extends Comment {
  user: UserSummary;
}

export interface CreateCommentRequest {
  content: string;
}
