'use client';

import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { apiClient } from '@/lib/api/client';
import { USERS } from '@/lib/api/endpoints';
import { useAuthStore } from '@/stores/auth-store';
import type { MyPageResponse, UpdateUserRequest, LinkedSocialAccount } from '@/types/user';

export function useMyPage() {
  return useQuery({
    queryKey: ['my-page'],
    queryFn: () => apiClient<MyPageResponse>(USERS.ME),
  });
}

export function useUpdateProfile() {
  const queryClient = useQueryClient();
  const { user, setUser } = useAuthStore();

  return useMutation({
    mutationFn: (data: UpdateUserRequest) =>
      apiClient<MyPageResponse>(USERS.UPDATE(user!.id), {
        method: 'PUT',
        body: data,
      }),
    onSuccess: (updated) => {
      queryClient.invalidateQueries({ queryKey: ['my-page'] });
      if (user) {
        setUser({ ...user, name: updated.name, avatar: updated.avatar });
      }
      toast.success('プロフィールを更新しました');
    },
    onError: () => {
      toast.error('プロフィールの更新に失敗しました');
    },
  });
}

export function useSocialAccounts() {
  return useQuery({
    queryKey: ['social-accounts'],
    queryFn: () => apiClient<LinkedSocialAccount[]>(USERS.MY_SOCIAL_ACCOUNTS),
  });
}

export function useUnlinkSocial() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (provider: string) =>
      apiClient(USERS.UNLINK_SOCIAL(provider), { method: 'DELETE' }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['social-accounts'] });
      toast.success('アカウント連携を解除しました');
    },
    onError: () => {
      toast.error('連携解除に失敗しました');
    },
  });
}
