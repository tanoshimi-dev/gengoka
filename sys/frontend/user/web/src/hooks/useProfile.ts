'use client';

import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { apiClient, apiClientPaginated } from '@/lib/api/client';
import { USERS } from '@/lib/api/endpoints';
import type { UserProfile } from '@/types/user';
import type { AnswerWithDetails } from '@/types/answer';

export function useUserProfile(userId: string) {
  return useQuery({
    queryKey: ['profile', userId],
    queryFn: () => apiClient<UserProfile>(USERS.GET(userId)),
    enabled: !!userId,
  });
}

export function useUserAnswers(userId: string, page: number = 1) {
  return useQuery({
    queryKey: ['profile-answers', userId, page],
    queryFn: () =>
      apiClientPaginated<AnswerWithDetails[]>(USERS.ANSWERS(userId), {
        params: { page, page_size: 10 },
      }),
    enabled: !!userId,
  });
}

export function useFollowUser(userId: string) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: () =>
      apiClient(USERS.FOLLOW(userId), { method: 'POST' }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['profile', userId] });
      toast.success('フォローしました');
    },
    onError: () => {
      toast.error('フォローに失敗しました');
    },
  });
}

export function useUnfollowUser(userId: string) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: () =>
      apiClient(USERS.UNFOLLOW(userId), { method: 'DELETE' }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['profile', userId] });
      toast.success('フォローを解除しました');
    },
    onError: () => {
      toast.error('フォロー解除に失敗しました');
    },
  });
}
