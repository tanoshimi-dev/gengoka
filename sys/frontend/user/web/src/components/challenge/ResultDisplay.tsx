'use client';

import Link from 'next/link';
import { GradientButton } from '@/components/common/GradientButton';
import { ScoreCircle } from './ScoreCircle';
import type { Answer } from '@/types/answer';

interface ResultDisplayProps {
  answer: Answer;
}

function getScoreMessage(score: number): string {
  if (score >= 90) return '素晴らしい！';
  if (score >= 80) return 'とても良くできました！';
  if (score >= 70) return '良い回答です！';
  if (score >= 60) return 'もう少し工夫してみましょう';
  return 'がんばりましょう！';
}

export function ResultDisplay({ answer }: ResultDisplayProps) {
  const score = answer.ai_feedback?.score ?? answer.score ?? 0;
  const feedback = answer.ai_feedback;

  return (
    <div className="space-y-6">
      {/* Score */}
      <div className="flex flex-col items-center gap-2 rounded-xl bg-white p-6 shadow-sm">
        <ScoreCircle score={score} />
        <p className="mt-2 text-lg font-bold text-[#1a1a2e]">{getScoreMessage(score)}</p>
      </div>

      {/* AI Feedback */}
      {feedback && (
        <div className="space-y-3">
          {feedback.good_points && (
            <div className="rounded-xl border-l-4 border-[#34C759] bg-white p-4 shadow-sm">
              <p className="mb-1 text-sm font-semibold text-[#34C759]">良い点</p>
              <p className="whitespace-pre-wrap text-sm text-[#1a1a2e]">{feedback.good_points}</p>
            </div>
          )}
          {feedback.improvement && (
            <div className="rounded-xl border-l-4 border-[#FF9500] bg-white p-4 shadow-sm">
              <p className="mb-1 text-sm font-semibold text-[#FF9500]">改善点</p>
              <p className="whitespace-pre-wrap text-sm text-[#1a1a2e]">{feedback.improvement}</p>
            </div>
          )}
          {feedback.example_answer && (
            <div className="rounded-xl border-l-4 border-[#007AFF] bg-white p-4 shadow-sm">
              <p className="mb-1 text-sm font-semibold text-[#007AFF]">お手本回答</p>
              <p className="whitespace-pre-wrap text-sm text-[#1a1a2e]">{feedback.example_answer}</p>
            </div>
          )}
        </div>
      )}

      {/* Your answer */}
      <div className="rounded-xl bg-white p-4 shadow-sm">
        <p className="mb-2 text-sm font-semibold text-[#1a1a2e]">あなたの回答</p>
        <p className="whitespace-pre-wrap text-sm text-[#666666]">{answer.content}</p>
      </div>

      {/* Back to home */}
      <Link href="/home">
        <GradientButton className="w-full">ホームに戻る</GradientButton>
      </Link>
    </div>
  );
}
