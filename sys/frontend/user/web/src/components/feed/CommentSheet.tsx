'use client';

import { useState } from 'react';
import { Send, Trash2 } from 'lucide-react';
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetDescription,
} from '@/components/ui/sheet';
import { UserAvatar } from '@/components/common/UserAvatar';
import { LoadingSpinner } from '@/components/common/LoadingSpinner';
import { useComments, useCreateComment, useDeleteComment } from '@/hooks/useComments';
import { useAuthStore } from '@/stores/auth-store';
import { formatRelativeTime } from '@/lib/utils/date';

interface CommentSheetProps {
  answerId: string;
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export function CommentSheet({ answerId, open, onOpenChange }: CommentSheetProps) {
  const [page, setPage] = useState(1);
  const [text, setText] = useState('');
  const currentUser = useAuthStore((s) => s.user);

  const { data, isLoading } = useComments(answerId, page);
  const createComment = useCreateComment(answerId);
  const deleteComment = useDeleteComment(answerId);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!text.trim()) return;
    createComment.mutate(
      { content: text.trim() },
      { onSuccess: () => setText('') },
    );
  };

  return (
    <Sheet open={open} onOpenChange={onOpenChange}>
      <SheetContent side="bottom" className="h-[70vh] rounded-t-2xl">
        <SheetHeader className="border-b pb-3">
          <SheetTitle className="text-[#1a1a2e]">コメント</SheetTitle>
          <SheetDescription className="sr-only">コメント一覧と投稿</SheetDescription>
        </SheetHeader>

        <div className="flex-1 overflow-y-auto px-4 py-3">
          {isLoading ? (
            <LoadingSpinner />
          ) : data?.data.length === 0 ? (
            <p className="py-8 text-center text-sm text-[#999999]">
              まだコメントがありません
            </p>
          ) : (
            <div className="space-y-4">
              {data?.data.map((comment) => (
                <div key={comment.id} className="flex gap-3">
                  <UserAvatar
                    name={comment.user.name}
                    avatar={comment.user.avatar}
                    className="h-8 w-8 flex-shrink-0"
                  />
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2">
                      <span className="text-sm font-medium text-[#1a1a2e]">
                        {comment.user.name}
                      </span>
                      <span className="text-xs text-[#999999]">
                        {formatRelativeTime(comment.created_at)}
                      </span>
                      {currentUser?.id === comment.user_id && (
                        <button
                          onClick={() => deleteComment.mutate(comment.id)}
                          disabled={deleteComment.isPending}
                          className="ml-auto text-[#999999] hover:text-[#E53935]"
                        >
                          <Trash2 className="h-3.5 w-3.5" />
                        </button>
                      )}
                    </div>
                    <p className="mt-0.5 text-sm text-[#333333]">{comment.content}</p>
                  </div>
                </div>
              ))}

              {data?.pagination.has_more && (
                <button
                  onClick={() => setPage((p) => p + 1)}
                  className="w-full py-2 text-center text-sm text-[#667eea] hover:underline"
                >
                  もっと見る
                </button>
              )}
            </div>
          )}
        </div>

        <form
          onSubmit={handleSubmit}
          className="flex items-center gap-2 border-t px-4 py-3"
        >
          <input
            value={text}
            onChange={(e) => setText(e.target.value)}
            placeholder="コメントを入力..."
            className="flex-1 rounded-full border border-[#e0e0e0] bg-[#F8F9FA] px-4 py-2 text-sm outline-none focus:border-[#667eea]"
          />
          <button
            type="submit"
            disabled={!text.trim() || createComment.isPending}
            className="flex h-9 w-9 items-center justify-center rounded-full bg-gradient-to-r from-[#667eea] to-[#764ba2] text-white disabled:opacity-50"
          >
            <Send className="h-4 w-4" />
          </button>
        </form>
      </SheetContent>
    </Sheet>
  );
}
