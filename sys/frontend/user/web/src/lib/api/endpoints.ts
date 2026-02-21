// Auth
export const AUTH = {
  REGISTER: '/auth/register',
  LOGIN: '/auth/login',
  REFRESH: '/auth/refresh',
  LOGOUT: '/auth/logout',
  SOCIAL: '/auth/social',
} as const;

// Categories
export const CATEGORIES = {
  LIST: '/categories',
  GET: (id: string) => `/categories/${id}`,
  CHALLENGES: (id: string) => `/categories/${id}/challenges`,
} as const;

// Challenges
export const CHALLENGES = {
  LIST: '/challenges',
  DAILY: '/challenges/daily',
  GET: (id: string) => `/challenges/${id}`,
  ANSWERS: (id: string) => `/challenges/${id}/answers`,
} as const;

// Answers
export const ANSWERS = {
  GET: (id: string) => `/answers/${id}`,
  UPDATE: (id: string) => `/answers/${id}`,
  DELETE: (id: string) => `/answers/${id}`,
  LIKE: (id: string) => `/answers/${id}/like`,
  UNLIKE: (id: string) => `/answers/${id}/like`,
  COMMENTS: (id: string) => `/answers/${id}/comments`,
} as const;

// Comments
export const COMMENTS = {
  DELETE: (id: string) => `/comments/${id}`,
} as const;

// Users
export const USERS = {
  ME: '/users/me',
  MY_STATS: '/users/me/stats',
  MY_SOCIAL_ACCOUNTS: '/users/me/social-accounts',
  UNLINK_SOCIAL: (provider: string) => `/users/me/social-accounts/${provider}`,
  GET: (id: string) => `/users/${id}`,
  UPDATE: (id: string) => `/users/${id}`,
  ANSWERS: (id: string) => `/users/${id}/answers`,
  FOLLOW: (id: string) => `/users/${id}/follow`,
  UNFOLLOW: (id: string) => `/users/${id}/follow`,
  FOLLOWERS: (id: string) => `/users/${id}/followers`,
  FOLLOWING: (id: string) => `/users/${id}/following`,
} as const;

// Feed & Rankings
export const FEED = {
  LIST: '/feed',
  TRENDING: '/trending',
} as const;

export const RANKINGS = {
  DAILY: '/rankings/daily',
  WEEKLY: '/rankings/weekly',
  ALL_TIME: '/rankings/all-time',
} as const;
