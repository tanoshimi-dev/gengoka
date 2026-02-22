'use client';

import { useInfiniteQuery } from '@tanstack/react-query';
import { apiClient } from '@/lib/api/client';
import { FEED } from '@/lib/api/endpoints';
import type { ApiResponse, PaginationInfo } from '@/lib/api/types';
import type { AnswerWithDetails } from '@/types/answer';
import { useAuthStore } from '@/stores/auth-store';

const API_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080/api/v1';

interface FeedPage {
  data: AnswerWithDetails[];
  pagination: PaginationInfo;
}

async function fetchFeedPage(filter: string, page: number): Promise<FeedPage> {
  const { accessToken } = useAuthStore.getState();
  const params = new URLSearchParams();
  params.set('filter', filter);
  params.set('page', String(page));
  params.set('page_size', '20');

  const res = await fetch(`${API_URL}${FEED.LIST}?${params.toString()}`, {
    headers: {
      'Content-Type': 'application/json',
      ...(accessToken ? { Authorization: `Bearer ${accessToken}` } : {}),
    },
  });

  if (!res.ok) {
    throw new Error(`Request failed: ${res.status}`);
  }

  const json: ApiResponse<AnswerWithDetails[]> = await res.json();
  return {
    data: json.data ?? [],
    pagination: json.pagination!,
  };
}

export function useFeed(filter: 'all' | 'following') {
  return useInfiniteQuery({
    queryKey: ['feed', filter],
    queryFn: ({ pageParam }) => fetchFeedPage(filter, pageParam),
    initialPageParam: 1,
    getNextPageParam: (lastPage) =>
      lastPage.pagination.has_more ? lastPage.pagination.page + 1 : undefined,
  });
}
