'use client';

import { Heart } from 'lucide-react';
import { motion } from 'framer-motion';
import { useToggleLike } from '@/hooks/useLike';

interface LikeButtonProps {
  answerId: string;
  isLiked: boolean;
  likeCount: number;
}

export function LikeButton({ answerId, isLiked, likeCount }: LikeButtonProps) {
  const toggleLike = useToggleLike();

  const handleClick = () => {
    toggleLike.mutate({ answerId, isLiked });
  };

  return (
    <button
      onClick={handleClick}
      disabled={toggleLike.isPending}
      className="flex items-center gap-1 text-sm transition-colors"
    >
      <motion.div
        whileTap={{ scale: 1.3 }}
        transition={{ type: 'spring', stiffness: 400, damping: 10 }}
      >
        <Heart
          className={`h-5 w-5 ${
            isLiked ? 'fill-[#FF2D55] text-[#FF2D55]' : 'text-[#999999]'
          }`}
        />
      </motion.div>
      <span className={isLiked ? 'text-[#FF2D55]' : 'text-[#999999]'}>
        {likeCount}
      </span>
    </button>
  );
}
