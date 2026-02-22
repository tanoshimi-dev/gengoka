export interface UserSummary {
  id: string;
  name: string;
  avatar: string | null;
}

export interface UserProfile {
  id: string;
  name: string;
  avatar: string | null;
  bio: string | null;
  total_likes: number;
  answer_count: number;
  follower_count: number;
  following_count: number;
  is_following: boolean;
}

export interface UserStats {
  total_challenges: number;
  completed_today: number;
  current_streak: number;
  best_streak: number;
  average_score: number;
}

export interface MyPageResponse {
  id: string;
  email: string | null;
  name: string;
  avatar: string | null;
  bio: string | null;
  total_likes: number;
  answer_count: number;
  follower_count: number;
  following_count: number;
  total_challenges: number;
  completed_today: number;
  current_streak: number;
  best_streak: number;
  average_score: number;
}

export interface UpdateUserRequest {
  name?: string;
  avatar?: string;
  bio?: string;
}

export interface LinkedSocialAccount {
  provider: string;
  provider_email: string | null;
  provider_name: string | null;
  linked_at: string;
}
