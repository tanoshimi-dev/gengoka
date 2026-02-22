'use client';

import { useMutation, useQueryClient } from '@tanstack/react-query';
import { apiClient } from '@/lib/api/client';
import { ANSWERS } from '@/lib/api/endpoints';
import type { AnswerWithDetails } from '@/types/answer';

export function useToggleLike() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ answerId, isLiked }: { answerId: string; isLiked: boolean }) => {
      if (isLiked) {
        return apiClient(ANSWERS.UNLIKE(answerId), { method: 'DELETE' });
      }
      return apiClient(ANSWERS.LIKE(answerId), { method: 'POST' });
    },
    onMutate: async ({ answerId, isLiked }) => {
      await queryClient.cancelQueries({ queryKey: ['feed'] });
      await queryClient.cancelQueries({ queryKey: ['profile-answers'] });

      const previousFeed = queryClient.getQueriesData({ queryKey: ['feed'] });
      const previousProfileAnswers = queryClient.getQueriesData({ queryKey: ['profile-answers'] });

      const updateAnswer = (answer: AnswerWithDetails) => {
        if (answer.id !== answerId) return answer;
        return {
          ...answer,
          is_liked: !isLiked,
          like_count: isLiked ? answer.like_count - 1 : answer.like_count + 1,
        };
      };

      // Update feed cache
      queryClient.setQueriesData({ queryKey: ['feed'] }, (old: unknown) => {
        if (!old || typeof old !== 'object') return old;
        const data = old as { pages: { data: AnswerWithDetails[]; pagination: unknown }[]; pageParams: unknown[] };
        return {
          ...data,
          pages: data.pages.map((page) => ({
            ...page,
            data: page.data.map(updateAnswer),
          })),
        };
      });

      // Update profile answers cache
      queryClient.setQueriesData({ queryKey: ['profile-answers'] }, (old: unknown) => {
        if (!old || typeof old !== 'object' || !('data' in (old as object))) return old;
        const data = old as { data: AnswerWithDetails[] };
        return { ...data, data: data.data.map(updateAnswer) };
      });

      return { previousFeed, previousProfileAnswers };
    },
    onError: (_err, _vars, context) => {
      if (context?.previousFeed) {
        for (const [key, data] of context.previousFeed) {
          queryClient.setQueryData(key, data);
        }
      }
      if (context?.previousProfileAnswers) {
        for (const [key, data] of context.previousProfileAnswers) {
          queryClient.setQueryData(key, data);
        }
      }
    },
  });
}
