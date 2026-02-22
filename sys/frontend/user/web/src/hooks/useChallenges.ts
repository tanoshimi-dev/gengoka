'use client';

import { useRouter } from 'next/navigation';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { apiClient } from '@/lib/api/client';
import { ANSWERS, CHALLENGES } from '@/lib/api/endpoints';
import { ApiError } from '@/lib/api/types';
import type { Answer, CreateAnswerRequest } from '@/types/answer';
import type { ChallengeWithCategory, DailyChallengeResponse } from '@/types/challenge';

export function useDailyChallenges() {
  return useQuery({
    queryKey: ['daily-challenges'],
    queryFn: () => apiClient<DailyChallengeResponse[]>(CHALLENGES.DAILY),
  });
}

export function useChallengeDetail(id: string) {
  return useQuery({
    queryKey: ['challenge', id],
    queryFn: () => apiClient<ChallengeWithCategory>(CHALLENGES.GET(id)),
    enabled: !!id,
  });
}

export function useSubmitAnswer(challengeId: string) {
  const router = useRouter();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: CreateAnswerRequest) =>
      apiClient<Answer>(CHALLENGES.ANSWERS(challengeId), {
        method: 'POST',
        body: data,
      }),
    onSuccess: (answer) => {
      queryClient.invalidateQueries({ queryKey: ['daily-challenges'] });
      queryClient.invalidateQueries({ queryKey: ['user-stats'] });
      toast.success('回答を送信しました');
      router.push(`/challenges/${challengeId}/result?answerId=${answer.id}`);
    },
    onError: (error) => {
      if (error instanceof ApiError && error.statusCode === 409) {
        toast.error('この問題は回答済みです');
      } else {
        toast.error('送信に失敗しました');
      }
    },
  });
}

export function useAnswerDetail(answerId: string | null) {
  return useQuery({
    queryKey: ['answer', answerId],
    queryFn: () => apiClient<Answer>(ANSWERS.GET(answerId!)),
    enabled: !!answerId,
  });
}
