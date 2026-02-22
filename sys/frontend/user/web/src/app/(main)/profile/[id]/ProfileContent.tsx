'use client';

import { useParams } from 'next/navigation';
import { ProfileHeader } from '@/components/profile/ProfileHeader';
import { UserAnswerList } from '@/components/profile/UserAnswerList';
import { LoadingSpinner } from '@/components/common/LoadingSpinner';
import { ErrorMessage } from '@/components/common/ErrorMessage';
import { useUserProfile } from '@/hooks/useProfile';

export function ProfileContent() {
  const { id } = useParams<{ id: string }>();
  const { data: profile, isLoading, isError, refetch } = useUserProfile(id);

  if (isLoading) return <LoadingSpinner />;
  if (isError || !profile) return <ErrorMessage onRetry={() => refetch()} />;

  return (
    <div className="space-y-6">
      <ProfileHeader profile={profile} />
      <UserAnswerList userId={id} />
    </div>
  );
}
