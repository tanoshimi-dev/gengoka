'use client';

import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { cn } from '@/lib/utils';

interface UserAvatarProps {
  name: string;
  avatar?: string | null;
  className?: string;
}

export function UserAvatar({ name, avatar, className }: UserAvatarProps) {
  const initials = name
    .split(/\s/)
    .map((w) => w[0])
    .join('')
    .slice(0, 2)
    .toUpperCase();

  return (
    <Avatar className={cn('h-8 w-8', className)}>
      {avatar && <AvatarImage src={avatar} alt={name} />}
      <AvatarFallback className="bg-gradient-to-br from-[#667eea] to-[#764ba2] text-xs font-semibold text-white">
        {initials}
      </AvatarFallback>
    </Avatar>
  );
}
