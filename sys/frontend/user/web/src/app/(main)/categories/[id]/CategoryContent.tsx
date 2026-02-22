'use client';

import { useState } from 'react';
import { useParams } from 'next/navigation';
import Link from 'next/link';
import { ArrowLeft, ChevronLeft, ChevronRight } from 'lucide-react';
import { useCategoryDetail, useCategoryChallenges } from '@/hooks/useCategories';
import { CATEGORY_COLORS } from '@/lib/utils/constants';
import { LoadingSpinner } from '@/components/common/LoadingSpinner';
import { ErrorMessage } from '@/components/common/ErrorMessage';

export function CategoryContent() {
  const { id } = useParams<{ id: string }>();
  const [page, setPage] = useState(1);

  const { data: category, isLoading: catLoading, error: catError, refetch: catRefetch } = useCategoryDetail(id);
  const { data: challengesData, isLoading: chLoading } = useCategoryChallenges(id, page);

  if (catLoading) return <LoadingSpinner />;
  if (catError || !category) return <ErrorMessage onRetry={catRefetch} />;

  const color = category.color_hex || CATEGORY_COLORS[category.name] || '#667eea';
  const challenges = challengesData?.data ?? [];
  const pagination = challengesData?.pagination;

  return (
    <div className="mx-auto max-w-2xl space-y-6">
      {/* Back */}
      <Link
        href="/home"
        className="inline-flex items-center gap-1 text-sm text-[#999999] hover:text-[#667eea]"
      >
        <ArrowLeft className="h-4 w-4" />
        戻る
      </Link>

      {/* Category Header */}
      <div className="flex items-start gap-4 rounded-xl bg-white p-4 shadow-sm">
        <span
          className="flex h-12 w-12 items-center justify-center rounded-lg text-xl"
          style={{ backgroundColor: `${color}20`, color }}
        >
          {category.icon_name || '📝'}
        </span>
        <div className="flex-1">
          <h1 className="text-lg font-bold text-[#1a1a2e]">{category.name}</h1>
          {category.description && (
            <p className="mt-1 text-sm text-[#666666]">{category.description}</p>
          )}
          <span className="mt-1 text-xs text-[#667eea]">{category.challenge_count}問</span>
        </div>
      </div>

      {/* Challenges List */}
      {chLoading ? (
        <LoadingSpinner />
      ) : challenges.length > 0 ? (
        <div className="space-y-3">
          {challenges.map((ch) => (
            <Link
              key={ch.id}
              href={`/challenges/${ch.id}`}
              className="block rounded-xl bg-white p-4 shadow-sm transition-colors hover:bg-[#F8F9FA]"
            >
              <p className="text-sm font-semibold text-[#1a1a2e]">{ch.title}</p>
              {ch.description && (
                <p className="mt-1 line-clamp-2 text-xs text-[#999999]">{ch.description}</p>
              )}
              <div className="mt-2 flex items-center gap-3 text-xs text-[#999999]">
                <span>{ch.char_limit}文字以内</span>
                <span>{ch.answer_count}回答</span>
              </div>
            </Link>
          ))}
        </div>
      ) : (
        <p className="py-8 text-center text-sm text-[#999999]">チャレンジがありません</p>
      )}

      {/* Pagination */}
      {pagination && pagination.total_pages > 1 && (
        <div className="flex items-center justify-center gap-4">
          <button
            onClick={() => setPage((p) => Math.max(1, p - 1))}
            disabled={page <= 1}
            className="flex items-center gap-1 rounded-lg px-3 py-2 text-sm text-[#667eea] disabled:opacity-40"
          >
            <ChevronLeft className="h-4 w-4" />
            Previous
          </button>
          <span className="text-sm text-[#999999]">
            {page} / {pagination.total_pages}
          </span>
          <button
            onClick={() => setPage((p) => p + 1)}
            disabled={!pagination.has_more}
            className="flex items-center gap-1 rounded-lg px-3 py-2 text-sm text-[#667eea] disabled:opacity-40"
          >
            Next
            <ChevronRight className="h-4 w-4" />
          </button>
        </div>
      )}
    </div>
  );
}
