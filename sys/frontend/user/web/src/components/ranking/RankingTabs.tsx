'use client';

import { cn } from '@/lib/utils';
import type { RankingPeriod } from '@/hooks/useRankings';

interface RankingTabsProps {
  period: RankingPeriod;
  onChange: (period: RankingPeriod) => void;
}

const tabs: { value: RankingPeriod; label: string }[] = [
  { value: 'daily', label: 'デイリー' },
  { value: 'weekly', label: 'ウィークリー' },
  { value: 'all-time', label: '累計' },
];

export function RankingTabs({ period, onChange }: RankingTabsProps) {
  return (
    <div className="flex gap-1 rounded-lg bg-white p-1 shadow-sm">
      {tabs.map((tab) => (
        <button
          key={tab.value}
          onClick={() => onChange(tab.value)}
          className={cn(
            'relative flex-1 rounded-md px-3 py-2 text-sm font-medium transition-all duration-200',
            period === tab.value
              ? 'text-white'
              : 'text-[#999999] hover:text-[#1a1a2e]',
          )}
        >
          {period === tab.value && (
            <span className="absolute inset-0 rounded-md bg-gradient-to-r from-[#667eea] to-[#764ba2]" />
          )}
          <span className="relative">{tab.label}</span>
        </button>
      ))}
    </div>
  );
}
