'use client';

import { useParams } from 'next/navigation';
import Link from 'next/link';
import { ArrowLeft } from 'lucide-react';
import { useChallengeDetail } from '@/hooks/useChallenges';
import { useDailyChallenges } from '@/hooks/useChallenges';
import { AnswerForm } from '@/components/challenge/AnswerForm';
import { LoadingSpinner } from '@/components/common/LoadingSpinner';
import { ErrorMessage } from '@/components/common/ErrorMessage';
import { CATEGORY_COLORS } from '@/lib/utils/constants';

export function ChallengeContent() {
  const { id } = useParams<{ id: string }>();
  const { data: challenge, isLoading, error, refetch } = useChallengeDetail(id);
  const { data: dailyChallenges } = useDailyChallenges();

  if (isLoading) return <LoadingSpinner />;
  if (error || !challenge) return <ErrorMessage onRetry={refetch} />;

  const categoryName = challenge.category?.name || '';
  const color = CATEGORY_COLORS[categoryName] || challenge.category?.color || '#667eea';

  // Find user_answer from daily challenges if available
  const dailyMatch = dailyChallenges?.find((d) => d.challenge.id === id);
  const userAnswer = dailyMatch?.user_answer ?? null;

  return (
    <div className="mx-auto max-w-2xl space-y-6">
      {/* Header */}
      <div>
        <Link
          href="/home"
          className="mb-4 inline-flex items-center gap-1 text-sm text-[#999999] hover:text-[#667eea]"
        >
          <ArrowLeft className="h-4 w-4" />
          戻る
        </Link>
        <div className="flex items-center gap-2">
          <span
            className="rounded-full px-2.5 py-0.5 text-xs font-medium text-white"
            style={{ backgroundColor: color }}
          >
            {categoryName}
          </span>
        </div>
        <h1 className="mt-2 text-xl font-bold text-[#1a1a2e]">{challenge.title}</h1>
        {challenge.description && (
          <p className="mt-2 text-sm text-[#666666]">{challenge.description}</p>
        )}
      </div>

      {/* Answer Form */}
      <AnswerForm challenge={challenge} userAnswer={userAnswer} />
    </div>
  );
}
