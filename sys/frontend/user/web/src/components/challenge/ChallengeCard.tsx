'use client';

import Link from 'next/link';
import { CheckCircle2 } from 'lucide-react';
import type { DailyChallengeResponse } from '@/types/challenge';
import { CATEGORY_COLORS } from '@/lib/utils/constants';

interface ChallengeCardProps {
  item: DailyChallengeResponse;
}

export function ChallengeCard({ item }: ChallengeCardProps) {
  const { challenge, category_name, is_completed } = item;
  const color = CATEGORY_COLORS[category_name] || '#667eea';

  return (
    <Link
      href={`/challenges/${challenge.id}`}
      className="relative flex min-w-[160px] max-w-[200px] shrink-0 snap-start flex-col justify-between rounded-xl p-4 shadow-sm transition-transform hover:scale-[1.02] sm:min-w-[200px]"
      style={{
        background: `linear-gradient(135deg, ${color}20, ${color}08)`,
        height: 140,
      }}
    >
      {is_completed && (
        <div className="absolute right-2 top-2">
          <CheckCircle2 className="h-5 w-5 text-[#34C759]" />
        </div>
      )}
      <span
        className="w-fit rounded-full px-2 py-0.5 text-[10px] font-medium text-white"
        style={{ backgroundColor: color }}
      >
        {category_name}
      </span>
      <div className="mt-2 flex-1">
        <p className="line-clamp-2 text-sm font-semibold text-[#1a1a2e]">
          {challenge.title}
        </p>
      </div>
      <p className="text-xs text-[#999999]">{challenge.char_limit}文字以内</p>
    </Link>
  );
}
