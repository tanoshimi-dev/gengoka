'use client';

import { cn } from '@/lib/utils';

interface CharacterCounterProps {
  current: number;
  max: number;
}

export function CharacterCounter({ current, max }: CharacterCounterProps) {
  return (
    <p
      className={cn(
        'text-right text-sm',
        current === 0 && 'text-[#999999]',
        current > 0 && current <= max && 'text-[#34C759]',
        current > max && 'text-[#E53935]',
      )}
    >
      {current} / {max} 文字
    </p>
  );
}
