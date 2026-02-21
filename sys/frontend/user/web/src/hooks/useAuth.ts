'use client';

import { useRouter } from 'next/navigation';
import { useMutation } from '@tanstack/react-query';
import { apiClient } from '@/lib/api/client';
import { AUTH } from '@/lib/api/endpoints';
import { useAuthStore } from '@/stores/auth-store';
import type { AuthTokens, LoginRequest, SignupRequest, SocialLoginRequest } from '@/types/auth';

export function useAuth() {
  const router = useRouter();
  const { setTokens, clearAuth, refreshToken } = useAuthStore();

  const loginMutation = useMutation({
    mutationFn: (data: LoginRequest) =>
      apiClient<AuthTokens>(AUTH.LOGIN, { method: 'POST', body: data, noAuth: true }),
    onSuccess: (tokens) => {
      setTokens(tokens.access_token, tokens.refresh_token, tokens.user);
      router.push('/home');
    },
  });

  const registerMutation = useMutation({
    mutationFn: (data: SignupRequest) =>
      apiClient<AuthTokens>(AUTH.REGISTER, { method: 'POST', body: data, noAuth: true }),
    onSuccess: (tokens) => {
      setTokens(tokens.access_token, tokens.refresh_token, tokens.user);
      router.push('/home');
    },
  });

  const socialLoginMutation = useMutation({
    mutationFn: (data: SocialLoginRequest) =>
      apiClient<AuthTokens>(AUTH.SOCIAL, { method: 'POST', body: data, noAuth: true }),
    onSuccess: (tokens) => {
      setTokens(tokens.access_token, tokens.refresh_token, tokens.user);
      router.push('/home');
    },
  });

  const logoutMutation = useMutation({
    mutationFn: () =>
      apiClient(AUTH.LOGOUT, {
        method: 'POST',
        body: { refresh_token: refreshToken },
      }),
    onSettled: () => {
      clearAuth();
      router.push('/login');
    },
  });

  return {
    login: loginMutation.mutateAsync,
    register: registerMutation.mutateAsync,
    socialLogin: socialLoginMutation.mutateAsync,
    logout: logoutMutation.mutate,
    isLoggingIn: loginMutation.isPending,
    isRegistering: registerMutation.isPending,
    isSocialLogging: socialLoginMutation.isPending,
    isLoggingOut: logoutMutation.isPending,
    loginError: loginMutation.error,
    registerError: registerMutation.error,
  };
}
