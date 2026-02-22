'use client';

import { useState } from 'react';
import Link from 'next/link';
import { MessageCircle } from 'lucide-react';
import { UserAvatar } from '@/components/common/UserAvatar';
import { LikeButton } from './LikeButton';
import { CommentSheet } from './CommentSheet';
import { formatRelativeTime } from '@/lib/utils/date';
import type { AnswerWithDetails } from '@/types/answer';

interface FeedCardProps {
  answer: AnswerWithDetails;
}

export function FeedCard({ answer }: FeedCardProps) {
  const [commentOpen, setCommentOpen] = useState(false);

  return (
    <>
      <div className="rounded-xl bg-white p-3 shadow-sm sm:p-4">
        {/* Header */}
        <div className="flex items-center gap-3">
          <Link href={`/profile/${answer.user.id}`}>
            <UserAvatar name={answer.user.name} avatar={answer.user.avatar} />
          </Link>
          <div className="flex-1 min-w-0">
            <Link
              href={`/profile/${answer.user.id}`}
              className="text-sm font-medium text-[#1a1a2e] hover:underline"
            >
              {answer.user.name}
            </Link>
            <p className="text-xs text-[#999999]">
              {formatRelativeTime(answer.created_at)}
            </p>
          </div>
        </div>

        {/* Challenge badge */}
        {answer.challenge && (
          <div className="mt-3">
            <span className="inline-block rounded-full bg-gradient-to-r from-[#667eea] to-[#764ba2] px-2.5 py-0.5 text-xs font-medium text-white">
              {answer.challenge.title}
            </span>
          </div>
        )}

        {/* Answer content */}
        <p className="mt-2 text-sm leading-relaxed text-[#333333] line-clamp-3">
          {answer.content}
        </p>

        {/* Score badge */}
        {answer.score !== null && (
          <div className="mt-2">
            <span className="inline-flex items-center gap-1 rounded-full bg-gradient-to-r from-[#667eea]/10 to-[#764ba2]/10 px-2.5 py-0.5 text-xs font-semibold text-[#667eea]">
              {answer.score}点
            </span>
          </div>
        )}

        {/* Footer */}
        <div className="mt-3 flex items-center gap-4 border-t border-[#f0f0f0] pt-3">
          <LikeButton
            answerId={answer.id}
            isLiked={answer.is_liked}
            likeCount={answer.like_count}
          />
          <button
            onClick={() => setCommentOpen(true)}
            className="flex items-center gap-1 text-sm text-[#999999] transition-colors hover:text-[#667eea]"
          >
            <MessageCircle className="h-5 w-5" />
            <span>{answer.comment_count}</span>
          </button>
        </div>
      </div>

      <CommentSheet
        answerId={answer.id}
        open={commentOpen}
        onOpenChange={setCommentOpen}
      />
    </>
  );
}
