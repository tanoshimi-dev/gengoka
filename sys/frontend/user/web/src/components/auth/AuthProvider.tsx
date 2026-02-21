'use client';

import { useEffect, useRef } from 'react';
import { useAuthStore } from '@/stores/auth-store';
import { apiClient } from '@/lib/api/client';
import { AUTH } from '@/lib/api/endpoints';
import type { ApiResponse } from '@/lib/api/types';

const API_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080/api/v1';
const REFRESH_INTERVAL = 25 * 60 * 1000; // 25 minutes

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const intervalRef = useRef<ReturnType<typeof setInterval> | null>(null);

  useEffect(() => {
    const { refreshToken, setTokens, clearAuth, accessToken } = useAuthStore.getState();

    // On mount, if we have a refreshToken but no accessToken, refresh
    if (refreshToken && !accessToken) {
      fetch(`${API_URL}${AUTH.REFRESH}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ refresh_token: refreshToken }),
      })
        .then((res) => res.json())
        .then(
          (
            json: ApiResponse<{
              access_token: string;
              refresh_token: string;
              expires_in: number;
            }>,
          ) => {
            if (json.success && json.data) {
              setTokens(json.data.access_token, json.data.refresh_token, useAuthStore.getState().user);
            } else {
              clearAuth();
            }
          },
        )
        .catch(() => {
          clearAuth();
        });
    }

    // Set up periodic refresh
    intervalRef.current = setInterval(() => {
      const state = useAuthStore.getState();
      if (state.refreshToken && state.accessToken) {
        fetch(`${API_URL}${AUTH.REFRESH}`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ refresh_token: state.refreshToken }),
        })
          .then((res) => res.json())
          .then(
            (
              json: ApiResponse<{
                access_token: string;
                refresh_token: string;
                expires_in: number;
              }>,
            ) => {
              if (json.success && json.data) {
                state.setTokens(
                  json.data.access_token,
                  json.data.refresh_token,
                  state.user,
                );
              }
            },
          )
          .catch(() => {
            // Silent fail on periodic refresh
          });
      }
    }, REFRESH_INTERVAL);

    return () => {
      if (intervalRef.current) clearInterval(intervalRef.current);
    };
  }, []);

  return <>{children}</>;
}
