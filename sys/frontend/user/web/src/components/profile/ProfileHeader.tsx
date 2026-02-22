'use client';

import Link from 'next/link';
import { UserAvatar } from '@/components/common/UserAvatar';
import { GradientButton } from '@/components/common/GradientButton';
import { useAuthStore } from '@/stores/auth-store';
import { useFollowUser, useUnfollowUser } from '@/hooks/useProfile';
import type { UserProfile } from '@/types/user';

interface ProfileHeaderProps {
  profile: UserProfile;
}

const statItems = [
  { key: 'answer_count' as const, label: '回答' },
  { key: 'follower_count' as const, label: 'フォロワー' },
  { key: 'following_count' as const, label: 'フォロー中' },
];

export function ProfileHeader({ profile }: ProfileHeaderProps) {
  const currentUser = useAuthStore((s) => s.user);
  const isMe = currentUser?.id === profile.id;
  const follow = useFollowUser(profile.id);
  const unfollow = useUnfollowUser(profile.id);

  const handleToggleFollow = () => {
    if (profile.is_following) {
      unfollow.mutate();
    } else {
      follow.mutate();
    }
  };

  return (
    <div className="rounded-xl bg-white p-6 shadow-sm">
      <div className="flex items-start gap-4">
        <UserAvatar
          name={profile.name}
          avatar={profile.avatar}
          className="h-16 w-16 text-lg"
        />
        <div className="flex-1 min-w-0">
          <h1 className="text-xl font-bold text-[#1a1a2e]">{profile.name}</h1>
          {profile.bio && (
            <p className="mt-1 text-sm text-[#666666]">{profile.bio}</p>
          )}
        </div>
      </div>

      {/* Stats */}
      <div className="mt-4 grid grid-cols-3 gap-4 rounded-lg bg-[#F8F9FA] p-3">
        {statItems.map(({ key, label }) => (
          <div key={key} className="text-center">
            <div className="text-lg font-bold text-[#1a1a2e]">{profile[key]}</div>
            <div className="text-xs text-[#999999]">{label}</div>
          </div>
        ))}
      </div>

      {/* Action button */}
      <div className="mt-4">
        {isMe ? (
          <Link href="/mypage">
            <GradientButton variant="secondary" className="w-full">
              プロフィール編集
            </GradientButton>
          </Link>
        ) : (
          <GradientButton
            variant={profile.is_following ? 'secondary' : 'primary'}
            className="w-full"
            onClick={handleToggleFollow}
            loading={follow.isPending || unfollow.isPending}
          >
            {profile.is_following ? 'フォロー中' : 'フォローする'}
          </GradientButton>
        )}
      </div>
    </div>
  );
}
