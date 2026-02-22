'use client';

import Link from 'next/link';
import { Heart } from 'lucide-react';
import { UserAvatar } from '@/components/common/UserAvatar';
import { useAuthStore } from '@/stores/auth-store';
import type { AnswerWithDetails } from '@/types/answer';

interface RankingCardProps {
  answer: AnswerWithDetails;
  rank: number;
}

const medals: Record<number, string> = {
  1: '🥇',
  2: '🥈',
  3: '🥉',
};

export function RankingCard({ answer, rank }: RankingCardProps) {
  const currentUser = useAuthStore((s) => s.user);
  const isMe = currentUser?.id === answer.user_id;
  const medal = medals[rank];

  return (
    <div
      className={`flex items-center gap-2 rounded-xl p-2 shadow-sm transition-colors sm:gap-3 sm:p-3 ${
        isMe
          ? 'bg-gradient-to-r from-[#667eea]/10 to-[#764ba2]/10 ring-1 ring-[#667eea]/30'
          : 'bg-white'
      }`}
    >
      {/* Rank */}
      <div className="flex w-10 flex-shrink-0 items-center justify-center">
        {medal ? (
          <span className="text-xl">{medal}</span>
        ) : (
          <span className="text-sm font-bold text-[#999999]">{rank}</span>
        )}
      </div>

      {/* User */}
      <Link href={`/profile/${answer.user.id}`} className="flex-shrink-0">
        <UserAvatar
          name={answer.user.name}
          avatar={answer.user.avatar}
          className={rank <= 3 ? 'h-10 w-10' : 'h-8 w-8'}
        />
      </Link>

      {/* Content */}
      <div className="flex-1 min-w-0">
        <div className="flex items-center gap-2">
          <Link
            href={`/profile/${answer.user.id}`}
            className="truncate text-sm font-medium text-[#1a1a2e] hover:underline"
          >
            {answer.user.name}
          </Link>
          {isMe && (
            <span className="flex-shrink-0 rounded-full bg-gradient-to-r from-[#667eea] to-[#764ba2] px-2 py-0.5 text-[10px] font-semibold text-white">
              あなた
            </span>
          )}
        </div>
        {answer.challenge && (
          <p className="truncate text-xs text-[#999999]">{answer.challenge.title}</p>
        )}
        <p className="mt-0.5 truncate text-xs text-[#666666]">{answer.content}</p>
      </div>

      {/* Score */}
      <div className="flex flex-shrink-0 flex-col items-end gap-0.5">
        {answer.score !== null && (
          <span className="text-sm font-bold text-[#1a1a2e]">{answer.score}点</span>
        )}
        <span className="flex items-center gap-1 text-xs text-[#FF2D55]">
          <Heart className="h-3 w-3 fill-[#FF2D55]" />
          {answer.like_count}
        </span>
      </div>
    </div>
  );
}
