import { describe, it, expect } from 'vitest';
import { AUTH, CATEGORIES, CHALLENGES, ANSWERS, COMMENTS, USERS, FEED, RANKINGS } from '@/lib/api/endpoints';

describe('AUTH endpoints', () => {
  it('has correct static paths', () => {
    expect(AUTH.REGISTER).toBe('/auth/register');
    expect(AUTH.LOGIN).toBe('/auth/login');
    expect(AUTH.REFRESH).toBe('/auth/refresh');
    expect(AUTH.LOGOUT).toBe('/auth/logout');
    expect(AUTH.SOCIAL).toBe('/auth/social');
  });
});

describe('CATEGORIES endpoints', () => {
  it('has correct static paths', () => {
    expect(CATEGORIES.LIST).toBe('/categories');
  });

  it('generates correct dynamic paths', () => {
    expect(CATEGORIES.GET('abc')).toBe('/categories/abc');
    expect(CATEGORIES.CHALLENGES('xyz')).toBe('/categories/xyz/challenges');
  });
});

describe('CHALLENGES endpoints', () => {
  it('has correct static paths', () => {
    expect(CHALLENGES.LIST).toBe('/challenges');
    expect(CHALLENGES.DAILY).toBe('/challenges/daily');
  });

  it('generates correct dynamic paths', () => {
    expect(CHALLENGES.GET('123')).toBe('/challenges/123');
    expect(CHALLENGES.ANSWERS('456')).toBe('/challenges/456/answers');
  });
});

describe('ANSWERS endpoints', () => {
  it('generates correct dynamic paths', () => {
    const id = 'answer-1';
    expect(ANSWERS.GET(id)).toBe('/answers/answer-1');
    expect(ANSWERS.LIKE(id)).toBe('/answers/answer-1/like');
    expect(ANSWERS.UNLIKE(id)).toBe('/answers/answer-1/like');
    expect(ANSWERS.COMMENTS(id)).toBe('/answers/answer-1/comments');
  });
});

describe('COMMENTS endpoints', () => {
  it('generates correct delete path', () => {
    expect(COMMENTS.DELETE('c1')).toBe('/comments/c1');
  });
});

describe('USERS endpoints', () => {
  it('has correct static paths', () => {
    expect(USERS.ME).toBe('/users/me');
    expect(USERS.MY_STATS).toBe('/users/me/stats');
    expect(USERS.MY_SOCIAL_ACCOUNTS).toBe('/users/me/social-accounts');
  });

  it('generates correct dynamic paths', () => {
    const id = 'user-1';
    expect(USERS.GET(id)).toBe('/users/user-1');
    expect(USERS.UPDATE(id)).toBe('/users/user-1');
    expect(USERS.ANSWERS(id)).toBe('/users/user-1/answers');
    expect(USERS.FOLLOW(id)).toBe('/users/user-1/follow');
    expect(USERS.UNFOLLOW(id)).toBe('/users/user-1/follow');
    expect(USERS.FOLLOWERS(id)).toBe('/users/user-1/followers');
    expect(USERS.FOLLOWING(id)).toBe('/users/user-1/following');
  });

  it('generates correct social account paths', () => {
    expect(USERS.UNLINK_SOCIAL('google')).toBe('/users/me/social-accounts/google');
    expect(USERS.UNLINK_SOCIAL('apple')).toBe('/users/me/social-accounts/apple');
    expect(USERS.UNLINK_SOCIAL('line')).toBe('/users/me/social-accounts/line');
  });
});

describe('FEED endpoints', () => {
  it('has correct paths', () => {
    expect(FEED.LIST).toBe('/feed');
    expect(FEED.TRENDING).toBe('/trending');
  });
});

describe('RANKINGS endpoints', () => {
  it('has correct paths', () => {
    expect(RANKINGS.DAILY).toBe('/rankings/daily');
    expect(RANKINGS.WEEKLY).toBe('/rankings/weekly');
    expect(RANKINGS.ALL_TIME).toBe('/rankings/all-time');
  });
});
