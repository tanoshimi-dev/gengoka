'use client';

import { useEffect, useRef, useState, useCallback } from 'react';
import { Trophy } from 'lucide-react';
import { RankingTabs } from '@/components/ranking/RankingTabs';
import { RankingCard } from '@/components/ranking/RankingCard';
import { LoadingSpinner } from '@/components/common/LoadingSpinner';
import { ErrorMessage } from '@/components/common/ErrorMessage';
import { useRankings, type RankingPeriod } from '@/hooks/useRankings';

function RankingSkeleton() {
  return (
    <div className="space-y-2">
      {Array.from({ length: 10 }).map((_, i) => (
        <div key={i} className="h-16 animate-pulse rounded-xl bg-white shadow-sm" />
      ))}
    </div>
  );
}

const periodLabels: Record<RankingPeriod, string> = {
  daily: '今日',
  weekly: '今週',
  'all-time': '全期間',
};

export function RankingsContent() {
  const [period, setPeriod] = useState<RankingPeriod>('daily');
  const { data, isLoading, isError, refetch, fetchNextPage, hasNextPage, isFetchingNextPage } =
    useRankings(period);
  const observerRef = useRef<HTMLDivElement>(null);

  const handleObserver = useCallback(
    (entries: IntersectionObserverEntry[]) => {
      const [entry] = entries;
      if (entry.isIntersecting && hasNextPage && !isFetchingNextPage) {
        fetchNextPage();
      }
    },
    [fetchNextPage, hasNextPage, isFetchingNextPage],
  );

  useEffect(() => {
    const el = observerRef.current;
    if (!el) return;
    const observer = new IntersectionObserver(handleObserver, { threshold: 0.1 });
    observer.observe(el);
    return () => observer.disconnect();
  }, [handleObserver]);

  const allItems = data?.pages.flatMap((page) => page.data) ?? [];

  // Calculate rank offset per page
  const getRank = (globalIndex: number) => globalIndex + 1;

  return (
    <div className="space-y-4">
      <RankingTabs period={period} onChange={setPeriod} />

      {isLoading ? (
        <RankingSkeleton />
      ) : isError ? (
        <ErrorMessage onRetry={() => refetch()} />
      ) : allItems.length === 0 ? (
        <div className="flex flex-col items-center justify-center py-16">
          <Trophy className="h-12 w-12 text-[#cccccc]" />
          <p className="mt-3 text-lg text-[#999999]">
            {periodLabels[period]}のランキングはまだありません
          </p>
        </div>
      ) : (
        <>
          <div className="space-y-2">
            {allItems.map((answer, index) => (
              <RankingCard key={answer.id} answer={answer} rank={getRank(index)} />
            ))}
          </div>

          {/* Infinite scroll trigger */}
          <div ref={observerRef} className="py-4">
            {isFetchingNextPage && <LoadingSpinner message="読み込み中..." />}
          </div>
        </>
      )}
    </div>
  );
}
