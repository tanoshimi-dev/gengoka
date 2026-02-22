'use client';

import { GoogleLogin } from '@react-oauth/google';
import { toast } from 'sonner';
import { useAuth } from '@/hooks/useAuth';

const LINE_CHANNEL_ID = process.env.NEXT_PUBLIC_LINE_CHANNEL_ID || '';
const APPLE_CLIENT_ID = process.env.NEXT_PUBLIC_APPLE_CLIENT_ID || '';

function generateRandomState() {
  const array = new Uint8Array(32);
  crypto.getRandomValues(array);
  return Array.from(array, (b) => b.toString(16).padStart(2, '0')).join('');
}

export function SocialLoginButtons() {
  const { socialLogin, isSocialLogging } = useAuth();

  const handleApple = () => {
    const state = generateRandomState();
    sessionStorage.setItem('apple_oauth_state', state);
    const redirectUri = `${window.location.origin}/api/auth/apple/callback`;
    const url = `https://appleid.apple.com/auth/authorize?response_type=code%20id_token&response_mode=form_post&client_id=${APPLE_CLIENT_ID}&redirect_uri=${encodeURIComponent(redirectUri)}&state=${state}&scope=name%20email`;
    window.location.href = url;
  };

  const handleLine = () => {
    const state = generateRandomState();
    sessionStorage.setItem('line_oauth_state', state);
    const redirectUri = `${window.location.origin}/api/auth/line/callback`;
    const url = `https://access.line.me/oauth2/v2.1/authorize?response_type=code&client_id=${LINE_CHANNEL_ID}&redirect_uri=${encodeURIComponent(redirectUri)}&state=${state}&scope=profile`;
    window.location.href = url;
  };

  return (
    <div className="space-y-3">
      <div className="flex w-full justify-center [&>div]:w-full">
        <GoogleLogin
          onSuccess={async (credentialResponse) => {
            if (!credentialResponse.credential) {
              toast.error('Googleログインに失敗しました');
              return;
            }
            try {
              await socialLogin({
                provider: 'google',
                id_token: credentialResponse.credential,
              });
            } catch {
              toast.error('Googleログインに失敗しました');
            }
          }}
          onError={() => {
            toast.error('Googleログインに失敗しました');
          }}
          size="large"
          width={400}
          text="signin_with"
          shape="rectangular"
        />
      </div>

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
