'use client';

import { useDailyChallenges } from '@/hooks/useChallenges';
import { ChallengeCard } from './ChallengeCard';
import { PartyPopper } from 'lucide-react';

export function DailyChallengeList() {
  const { data, isLoading } = useDailyChallenges();

  const incomplete = data?.filter((d) => !d.is_completed).length ?? 0;

  return (
    <section>
      <div className="mb-3 flex items-center gap-2">
        <h2 className="text-lg font-bold text-[#1a1a2e]">今日のチャレンジ</h2>
        {!isLoading && incomplete > 0 && (
          <span className="rounded-full bg-[#667eea] px-2 py-0.5 text-xs font-medium text-white">
            残り {incomplete}
          </span>
        )}
      </div>

      {isLoading ? (
        <div className="-mx-4 flex gap-3 overflow-hidden px-4">
          {Array.from({ length: 3 }).map((_, i) => (
            <div
              key={i}
              className="min-w-[160px] max-w-[200px] shrink-0 animate-pulse rounded-xl bg-white shadow-sm sm:min-w-[200px]"
              style={{ height: 140 }}
            />
          ))}
        </div>
      ) : data && data.length > 0 ? (
        <div className="-mx-4 flex snap-x snap-mandatory gap-3 overflow-x-auto px-4 pb-2">
          {data.map((item) => (
            <ChallengeCard key={item.challenge.id} item={item} />
          ))}
        </div>
      ) : (
        <div className="flex flex-col items-center gap-2 rounded-xl bg-white py-8 shadow-sm">
          <PartyPopper className="h-8 w-8 text-[#FFD700]" />
          <p className="text-sm font-medium text-[#1a1a2e]">すべて完了！</p>
          <p className="text-xs text-[#999999]">今日のチャレンジはすべてクリアしました</p>
        </div>
      )}
    </section>
  );
}
