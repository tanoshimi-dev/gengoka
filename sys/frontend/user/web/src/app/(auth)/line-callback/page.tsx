'use client';

import { Suspense, useEffect, useRef } from 'react';
import { useRouter, useSearchParams } from 'next/navigation';
import { toast } from 'sonner';
import { useAuth } from '@/hooks/useAuth';

function LineCallbackContent() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const { socialLogin } = useAuth();
  const processedRef = useRef(false);

  useEffect(() => {
    if (processedRef.current) return;
    processedRef.current = true;

    const accessToken = searchParams.get('access_token');
    const state = searchParams.get('state');
    const savedState = sessionStorage.getItem('line_oauth_state');

    if (!accessToken || !state || state !== savedState) {
      toast.error('LINEログインに失敗しました');
      router.replace('/login');
      return;
    }

    sessionStorage.removeItem('line_oauth_state');

    socialLogin({ provider: 'line', access_token: accessToken })
      .catch(() => {
        toast.error('LINEログインに失敗しました');
        router.replace('/login');
      });
  }, [searchParams, socialLogin, router]);

  return (
    <div className="flex flex-col items-center gap-4">
      <div className="h-8 w-8 animate-spin rounded-full border-4 border-[#06C755] border-t-transparent" />
      <p className="text-sm text-gray-500">LINEでログイン中...</p>
    </div>
  );
}

export default function LineCallbackPage() {
  return (
    <Suspense
      fallback={
        <div className="flex flex-col items-center gap-4">
          <div className="h-8 w-8 animate-spin rounded-full border-4 border-[#06C755] border-t-transparent" />
          <p className="text-sm text-gray-500">LINEでログイン中...</p>
        </div>
      }
    >
      <LineCallbackContent />
    </Suspense>
  );
}
