'use client';

import { useSearchParams } from 'next/navigation';
import { useAnswerDetail } from '@/hooks/useChallenges';
import { ResultDisplay } from '@/components/challenge/ResultDisplay';
import { LoadingSpinner } from '@/components/common/LoadingSpinner';
import { ErrorMessage } from '@/components/common/ErrorMessage';

export function ResultContent() {
  const searchParams = useSearchParams();
  const answerId = searchParams.get('answerId');
  const { data: answer, isLoading, error, refetch } = useAnswerDetail(answerId);

  if (!answerId) return <ErrorMessage message="回答IDが見つかりません" />;
  if (isLoading) return <LoadingSpinner />;
  if (error || !answer) return <ErrorMessage onRetry={refetch} />;

  return (
    <div className="mx-auto max-w-2xl">
      <ResultDisplay answer={answer} />
    </div>
  );
}
