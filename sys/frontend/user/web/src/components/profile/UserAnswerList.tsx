'use client';

import { useState } from 'react';
import { ChevronLeft, ChevronRight } from 'lucide-react';
import { FeedCard } from '@/components/feed/FeedCard';
import { LoadingSpinner } from '@/components/common/LoadingSpinner';
import { useUserAnswers } from '@/hooks/useProfile';

interface UserAnswerListProps {
  userId: string;
}

export function UserAnswerList({ userId }: UserAnswerListProps) {
  const [page, setPage] = useState(1);
  const { data, isLoading } = useUserAnswers(userId, page);

  if (isLoading) return <LoadingSpinner />;
  if (!data || data.data.length === 0) {
    return (
      <p className="py-8 text-center text-sm text-[#999999]">
        まだ回答がありません
      </p>
    );
  }

  return (
    <div className="space-y-4">
      <h2 className="text-lg font-bold text-[#1a1a2e]">回答一覧</h2>

      <div className="space-y-3">
        {data.data.map((answer) => (
          <FeedCard key={answer.id} answer={answer} />
        ))}
      </div>

      {data.pagination.total_pages > 1 && (
        <div className="flex items-center justify-center gap-4 pt-2">
          <button
            onClick={() => setPage((p) => Math.max(1, p - 1))}
            disabled={page <= 1}
            className="flex items-center gap-1 text-sm text-[#667eea] disabled:text-[#cccccc]"
          >
            <ChevronLeft className="h-4 w-4" />
            前へ
          </button>
          <span className="text-sm text-[#999999]">
            {page} / {data.pagination.total_pages}
          </span>
          <button
            onClick={() => setPage((p) => p + 1)}
            disabled={!data.pagination.has_more}
            className="flex items-center gap-1 text-sm text-[#667eea] disabled:text-[#cccccc]"
          >
            次へ
            <ChevronRight className="h-4 w-4" />
          </button>
        </div>
      )}
    </div>
  );
}
