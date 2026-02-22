'use client';

import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { apiClient, apiClientPaginated } from '@/lib/api/client';
import { ANSWERS, COMMENTS } from '@/lib/api/endpoints';
import type { CommentWithUser, CreateCommentRequest } from '@/types/comment';

export function useComments(answerId: string, page: number = 1) {
  return useQuery({
    queryKey: ['comments', answerId, page],
    queryFn: () =>
      apiClientPaginated<CommentWithUser[]>(ANSWERS.COMMENTS(answerId), {
        params: { page, page_size: 20 },
      }),
    enabled: !!answerId,
  });
}

export function useCreateComment(answerId: string) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: CreateCommentRequest) =>
      apiClient<CommentWithUser>(ANSWERS.COMMENTS(answerId), {
        method: 'POST',
        body: data,
      }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['comments', answerId] });
      // Update comment_count in feed cache
      queryClient.invalidateQueries({ queryKey: ['feed'] });
    },
    onError: () => {
      toast.error('コメントの投稿に失敗しました');
    },
  });
}

export function useDeleteComment(answerId: string) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (commentId: string) =>
      apiClient(COMMENTS.DELETE(commentId), { method: 'DELETE' }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['comments', answerId] });
      queryClient.invalidateQueries({ queryKey: ['feed'] });
      toast.success('コメントを削除しました');
    },
    onError: () => {
      toast.error('コメントの削除に失敗しました');
    },
  });
}
