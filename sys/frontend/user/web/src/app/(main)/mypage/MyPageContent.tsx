'use client';

import { LogOut } from 'lucide-react';
import { UserAvatar } from '@/components/common/UserAvatar';
import { LoadingSpinner } from '@/components/common/LoadingSpinner';
import { ErrorMessage } from '@/components/common/ErrorMessage';
import { GradientButton } from '@/components/common/GradientButton';
import { StatsSection } from '@/components/mypage/StatsSection';
import { ProfileEditForm } from '@/components/mypage/ProfileEditForm';
import { SocialAccounts } from '@/components/mypage/SocialAccounts';
import { useMyPage } from '@/hooks/useMyPage';
import { useAuth } from '@/hooks/useAuth';

export function MyPageContent() {
  const { data, isLoading, isError, refetch } = useMyPage();
  const { logout, isLoggingOut } = useAuth();

  if (isLoading) return <LoadingSpinner />;
  if (isError || !data) return <ErrorMessage onRetry={() => refetch()} />;

  return (
    <div className="space-y-6">
      {/* Avatar & name header */}
      <div className="flex items-center gap-4 rounded-xl bg-white p-4 shadow-sm">
        <UserAvatar
          name={data.name}
          avatar={data.avatar}
          className="h-16 w-16 text-lg"
        />
        <div>
          <h1 className="text-xl font-bold text-[#1a1a2e]">{data.name}</h1>
          {data.email && (
            <p className="text-sm text-[#999999]">{data.email}</p>
          )}
        </div>
      </div>

      {/* Profile edit */}
      <div className="rounded-xl bg-[#F8F9FA] p-4">
        <ProfileEditForm
          defaultValues={{ name: data.name, bio: data.bio ?? '' }}
        />
      </div>

      {/* Stats */}
      <StatsSection data={data} />

      {/* Social accounts */}
      <div className="rounded-xl bg-[#F8F9FA] p-4">
        <SocialAccounts />
      </div>

      {/* Logout */}
      <GradientButton
        variant="secondary"
        className="w-full text-[#E53935] border-[#E53935] hover:bg-[#E53935]/5"
        onClick={() => logout()}
        loading={isLoggingOut}
      >
        <LogOut className="mr-2 h-4 w-4" />
        ログアウト
      </GradientButton>
    </div>
  );
}
