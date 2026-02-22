'use client';

import { useQuery } from '@tanstack/react-query';
import { apiClient, apiClientPaginated } from '@/lib/api/client';
import { CATEGORIES } from '@/lib/api/endpoints';
import type { CategoryResponse } from '@/types/category';
import type { Challenge } from '@/types/challenge';

export function useCategories() {
  return useQuery({
    queryKey: ['categories'],
    queryFn: () => apiClient<CategoryResponse[]>(CATEGORIES.LIST, { noAuth: true }),
  });
}

export function useCategoryDetail(id: string) {
  return useQuery({
    queryKey: ['category', id],
    queryFn: () => apiClient<CategoryResponse>(CATEGORIES.GET(id), { noAuth: true }),
    enabled: !!id,
  });
}

export function useCategoryChallenges(categoryId: string, page: number = 1) {
  return useQuery({
    queryKey: ['category-challenges', categoryId, page],
    queryFn: () =>
      apiClientPaginated<Challenge[]>(CATEGORIES.CHALLENGES(categoryId), {
        noAuth: true,
        params: { page, page_size: 10 },
      }),
    enabled: !!categoryId,
  });
}
