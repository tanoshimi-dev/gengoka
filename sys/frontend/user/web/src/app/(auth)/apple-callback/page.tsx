'use client';

import { Suspense, useEffect, useRef } from 'react';
import { useRouter, useSearchParams } from 'next/navigation';
import { toast } from 'sonner';
import { useAuth } from '@/hooks/useAuth';

function AppleCallbackContent() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const { socialLogin } = useAuth();
  const processedRef = useRef(false);

  useEffect(() => {
    if (processedRef.current) return;
    processedRef.current = true;

    const idToken = searchParams.get('id_token');
    const state = searchParams.get('state');
    const savedState = sessionStorage.getItem('apple_oauth_state');

    if (!idToken || !state || state !== savedState) {
      toast.error('Appleログインに失敗しました');
      router.replace('/login');
      return;
    }

    sessionStorage.removeItem('apple_oauth_state');

    socialLogin({ provider: 'apple', id_token: idToken })
      .catch(() => {
        toast.error('Appleログインに失敗しました');
        router.replace('/login');
      });
  }, [searchParams, socialLogin, router]);

  return (
    <div className="flex flex-col items-center gap-4">
      <div className="h-8 w-8 animate-spin rounded-full border-4 border-black border-t-transparent" />
      <p className="text-sm text-gray-500">Appleでログイン中...</p>
    </div>
  );
}

export default function AppleCallbackPage() {
  return (
    <Suspense
      fallback={
        <div className="flex flex-col items-center gap-4">
          <div className="h-8 w-8 animate-spin rounded-full border-4 border-black border-t-transparent" />
          <p className="text-sm text-gray-500">Appleでログイン中...</p>
        </div>
      }
    >
      <AppleCallbackContent />
    </Suspense>
  );
}
