'use client';

import { useAuth } from '@/hooks/useAuth';

export function SocialLoginButtons() {
  const { socialLogin, isSocialLogging } = useAuth();

  const handleGoogle = async () => {
    // TODO: Implement Google OAuth flow with NextAuth
    // For now, placeholder that will be connected later
    console.log('Google login - to be implemented with NextAuth');
  };

  const handleApple = async () => {
    console.log('Apple login - to be implemented with NextAuth');
  };

  const handleLine = async () => {
    console.log('LINE login - to be implemented with NextAuth');
  };

  return (
    <div className="space-y-3">
      <button
        type="button"
        onClick={handleGoogle}
        disabled={isSocialLogging}
        className="flex w-full items-center justify-center gap-3 rounded-lg border border-[#e0e0e0] bg-white px-4 py-3 text-sm font-medium text-[#1a1a2e] transition-colors hover:bg-gray-50 disabled:opacity-50"
      >
        <svg className="h-5 w-5" viewBox="0 0 24 24">
          <path
            d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92a5.06 5.06 0 0 1-2.2 3.32v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.1z"
            fill="#4285F4"
          />
          <path
            d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"
            fill="#34A853"
          />
          <path
            d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"
            fill="#FBBC05"
          />
          <path
            d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"
            fill="#EA4335"
          />
        </svg>
        Googleでログイン
      </button>

      <button
        type="button"
        onClick={handleApple}
        disabled={isSocialLogging}
        className="flex w-full items-center justify-center gap-3 rounded-lg border border-[#e0e0e0] bg-black px-4 py-3 text-sm font-medium text-white transition-colors hover:bg-gray-900 disabled:opacity-50"
      >
        <svg className="h-5 w-5" viewBox="0 0 24 24" fill="currentColor">
          <path d="M17.05 20.28c-.98.95-2.05.88-3.08.4-1.09-.5-2.08-.48-3.24 0-1.44.62-2.2.44-3.06-.4C2.79 15.25 3.51 7.59 9.05 7.31c1.35.07 2.29.74 3.08.8 1.18-.24 2.31-.93 3.57-.84 1.51.12 2.65.72 3.4 1.8-3.12 1.87-2.38 5.98.48 7.13-.57 1.5-1.31 2.99-2.54 4.09zM12.03 7.25c-.15-2.23 1.66-4.07 3.74-4.25.29 2.58-2.34 4.5-3.74 4.25z" />
        </svg>
        Appleでログイン
      </button>

      <button
        type="button"
        onClick={handleLine}
        disabled={isSocialLogging}
        className="flex w-full items-center justify-center gap-3 rounded-lg border border-[#e0e0e0] bg-[#06C755] px-4 py-3 text-sm font-medium text-white transition-colors hover:bg-[#05b34c] disabled:opacity-50"
      >
        <svg className="h-5 w-5" viewBox="0 0 24 24" fill="currentColor">
          <path d="M19.365 9.863c.349 0 .63.285.63.631 0 .345-.281.63-.63.63H17.61v1.125h1.755c.349 0 .63.283.63.63 0 .344-.281.629-.63.629h-2.386a.63.63 0 0 1-.63-.629V8.108a.63.63 0 0 1 .63-.63h2.386c.349 0 .63.285.63.63 0 .349-.281.63-.63.63H17.61v1.125h1.755zm-3.855 3.016a.63.63 0 0 1-.63.63.626.626 0 0 1-.51-.262l-2.397-3.274v2.906a.63.63 0 0 1-.629.63.63.63 0 0 1-.63-.63V8.108a.63.63 0 0 1 .63-.63c.2 0 .385.096.504.259l2.397 3.274V8.108a.63.63 0 0 1 1.265 0v4.771zm-5.741 0a.63.63 0 0 1-1.26 0V8.108a.63.63 0 0 1 1.26 0v4.771zm-2.451.63H4.932a.63.63 0 0 1-.63-.63V8.108a.63.63 0 0 1 1.261 0v4.141h1.756a.63.63 0 1 1 0 1.26zM24 10.314C24 4.943 18.615.572 12 .572S0 4.943 0 10.314c0 4.811 4.27 8.842 10.035 9.608.391.082.923.258 1.058.59.12.301.079.766.038 1.08l-.164 1.02c-.045.301-.24 1.186 1.049.645 1.291-.539 6.916-4.078 9.436-6.975C23.176 14.393 24 12.458 24 10.314" />
        </svg>
        LINEでログイン
      </button>
    </div>
  );
}
