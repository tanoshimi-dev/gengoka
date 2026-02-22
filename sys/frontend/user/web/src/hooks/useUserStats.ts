'use client';

import { useQuery } from '@tanstack/react-query';
import { apiClient } from '@/lib/api/client';
import { USERS } from '@/lib/api/endpoints';
import { useAuthStore } from '@/stores/auth-store';
import type { UserStats } from '@/types/user';

export function useUserStats() {
  const accessToken = useAuthStore((s) => s.accessToken);

  return useQuery({
    queryKey: ['user-stats'],
    queryFn: () => apiClient<UserStats>(USERS.MY_STATS),
    enabled: !!accessToken,
  });
}
