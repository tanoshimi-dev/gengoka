'use client';

import { useInfiniteQuery } from '@tanstack/react-query';
import { RANKINGS } from '@/lib/api/endpoints';
import { useAuthStore } from '@/stores/auth-store';
import type { ApiResponse, PaginationInfo } from '@/lib/api/types';
import type { AnswerWithDetails } from '@/types/answer';

export type RankingPeriod = 'daily' | 'weekly' | 'all-time';

const API_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080/api/v1';

const endpointMap: Record<RankingPeriod, string> = {
  daily: RANKINGS.DAILY,
  weekly: RANKINGS.WEEKLY,
  'all-time': RANKINGS.ALL_TIME,
};

interface RankingPage {
  data: AnswerWithDetails[];
  pagination: PaginationInfo;
}

async function fetchRankingPage(period: RankingPeriod, page: number): Promise<RankingPage> {
  const { accessToken } = useAuthStore.getState();
  const params = new URLSearchParams();
  params.set('page', String(page));
  params.set('page_size', '20');

  const res = await fetch(`${API_URL}${endpointMap[period]}?${params.toString()}`, {
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

export function useRankings(period: RankingPeriod) {
  return useInfiniteQuery({
    queryKey: ['rankings', period],
    queryFn: ({ pageParam }) => fetchRankingPage(period, pageParam),
    initialPageParam: 1,
    getNextPageParam: (lastPage) =>
      lastPage.pagination.has_more ? lastPage.pagination.page + 1 : undefined,
  });
}
