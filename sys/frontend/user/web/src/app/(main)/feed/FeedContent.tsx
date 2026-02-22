'use client';

import { useEffect, useRef, useState, useCallback } from 'react';
import { FilterTabs } from '@/components/feed/FilterTabs';
import { FeedCard } from '@/components/feed/FeedCard';
import { LoadingSpinner } from '@/components/common/LoadingSpinner';
import { ErrorMessage } from '@/components/common/ErrorMessage';
import { useFeed } from '@/hooks/useFeed';

function FeedSkeleton() {
  return (
    <div className="space-y-3">
      {Array.from({ length: 3 }).map((_, i) => (
        <div key={i} className="h-48 animate-pulse rounded-xl bg-white shadow-sm" />
      ))}
    </div>
  );
}

export function FeedContent() {
  const [filter, setFilter] = useState<'all' | 'following'>('all');
  const { data, isLoading, isError, refetch, fetchNextPage, hasNextPage, isFetchingNextPage } =
    useFeed(filter);
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

  return (
    <div className="space-y-4">
      <FilterTabs filter={filter} onChange={setFilter} />

      {isLoading ? (
        <FeedSkeleton />
      ) : isError ? (
        <ErrorMessage onRetry={() => refetch()} />
      ) : allItems.length === 0 ? (
        <div className="flex flex-col items-center justify-center py-16">
          <p className="text-lg text-[#999999]">まだ投稿がありません</p>
          {filter === 'following' && (
            <p className="mt-1 text-sm text-[#cccccc]">
              ユーザーをフォローするとここに表示されます
            </p>
          )}
        </div>
      ) : (
        <>
          <div className="space-y-3">
            {allItems.map((answer) => (
              <FeedCard key={answer.id} answer={answer} />
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
