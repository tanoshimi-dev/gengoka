'use client';

import { useState } from 'react';
import Link from 'next/link';
import { GradientButton } from '@/components/common/GradientButton';
import { CharacterCounter } from './CharacterCounter';
import { useSubmitAnswer } from '@/hooks/useChallenges';
import type { ChallengeWithCategory } from '@/types/challenge';
import type { Answer } from '@/types/answer';
import { CheckCircle2 } from 'lucide-react';

interface AnswerFormProps {
  challenge: ChallengeWithCategory;
  userAnswer?: Answer | null;
}

export function AnswerForm({ challenge, userAnswer }: AnswerFormProps) {
  const [content, setContent] = useState('');
  const { mutate: submit, isPending } = useSubmitAnswer(challenge.id);
  const isAnswered = !!userAnswer;

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!content.trim() || content.length > challenge.char_limit) return;
    submit({ content: content.trim() });
  };

  if (isAnswered) {
    return (
      <div className="space-y-3">
        <div className="flex items-center gap-2">
          <CheckCircle2 className="h-5 w-5 text-[#34C759]" />
          <span className="text-sm font-medium text-[#34C759]">回答済み</span>
        </div>
        <div className="rounded-xl bg-[#F8F9FA] p-4">
          <p className="whitespace-pre-wrap text-sm text-[#1a1a2e]">{userAnswer.content}</p>
        </div>
        <Link href={`/challenges/${challenge.id}/result?answerId=${userAnswer.id}`}>
          <GradientButton className="w-full">結果を見る</GradientButton>
        </Link>
      </div>
    );
  }

  return (
    <form onSubmit={handleSubmit} className="space-y-3">
      <textarea
        value={content}
        onChange={(e) => setContent(e.target.value)}
        placeholder="回答を入力してください..."
        rows={6}
        className="w-full resize-none rounded-xl border border-[#E0E0E0] bg-white p-4 text-sm text-[#1a1a2e] placeholder:text-[#BDBDBD] focus:border-[#667eea] focus:outline-none focus:ring-1 focus:ring-[#667eea]"
      />
      <CharacterCounter current={content.length} max={challenge.char_limit} />
      <GradientButton
        type="submit"
        className="w-full"
        loading={isPending}
        disabled={!content.trim() || content.length > challenge.char_limit}
      >
        回答を送信
      </GradientButton>
    </form>
  );
}
