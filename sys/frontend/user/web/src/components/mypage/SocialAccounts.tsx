'use client';

import { Loader2 } from 'lucide-react';
import { useSocialAccounts, useUnlinkSocial } from '@/hooks/useMyPage';
import { GradientButton } from '@/components/common/GradientButton';
import { LoadingSpinner } from '@/components/common/LoadingSpinner';
import { formatRelativeTime } from '@/lib/utils/date';

const providerLabels: Record<string, string> = {
  google: 'Google',
  apple: 'Apple',
  line: 'LINE',
};

const providerColors: Record<string, string> = {
  google: '#4285F4',
  apple: '#000000',
  line: '#06C755',
};

export function SocialAccounts() {
  const { data: accounts, isLoading } = useSocialAccounts();
  const unlinkSocial = useUnlinkSocial();

  if (isLoading) return <LoadingSpinner />;

  return (
    <div className="space-y-3">
      <h3 className="text-sm font-semibold text-[#1a1a2e]">アカウント連携</h3>

      {!accounts || accounts.length === 0 ? (
        <p className="text-sm text-[#999999]">連携中のアカウントはありません</p>
      ) : (
        <div className="space-y-2">
          {accounts.map((account) => (
            <div
              key={account.provider}
              className="flex items-center gap-3 rounded-xl bg-white p-4 shadow-sm"
            >
              <div
                className="flex h-10 w-10 items-center justify-center rounded-full text-white text-sm font-bold"
                style={{ backgroundColor: providerColors[account.provider] || '#667eea' }}
              >
                {(providerLabels[account.provider] || account.provider)[0]}
              </div>
              <div className="flex-1 min-w-0">
                <div className="text-sm font-medium text-[#1a1a2e]">
                  {providerLabels[account.provider] || account.provider}
                </div>
                {account.provider_email && (
                  <div className="truncate text-xs text-[#999999]">
                    {account.provider_email}
                  </div>
                )}
                <div className="text-xs text-[#cccccc]">
                  {formatRelativeTime(account.linked_at)}に連携
                </div>
              </div>
              <GradientButton
                variant="secondary"
                className="text-xs px-3 py-1.5"
                onClick={() => unlinkSocial.mutate(account.provider)}
                loading={unlinkSocial.isPending}
              >
                {unlinkSocial.isPending ? (
                  <Loader2 className="h-3 w-3 animate-spin" />
                ) : (
                  '連携解除'
                )}
              </GradientButton>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
