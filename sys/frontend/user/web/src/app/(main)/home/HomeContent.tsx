'use client';

import { StatsBar } from '@/components/home/StatsBar';
import { DailyChallengeList } from '@/components/challenge/DailyChallengeList';
import { CategoryGrid } from '@/components/category/CategoryGrid';

export function HomeContent() {
  return (
    <div className="space-y-6">
      <StatsBar />
      <DailyChallengeList />
      <CategoryGrid />
    </div>
  );
}
