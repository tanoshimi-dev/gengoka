import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import type { UserSummary } from '@/types/user';

interface AuthState {
  accessToken: string | null;
  refreshToken: string | null;
  user: UserSummary | null;
  setTokens: (accessToken: string, refreshToken: string, user: UserSummary | null) => void;
  setUser: (user: UserSummary) => void;
  clearAuth: () => void;
}

export const useAuthStore = create<AuthState>()(
  persist(
    (set) => ({
      accessToken: null,
      refreshToken: null,
      user: null,
      setTokens: (accessToken, refreshToken, user) => {
        // Set cookie for middleware auth check
        document.cookie = `gengoka-auth-present=1; path=/; max-age=${60 * 60 * 24 * 90}; SameSite=Lax`;
        set({ accessToken, refreshToken, user });
      },
      setUser: (user) => set({ user }),
      clearAuth: () => {
        // Remove cookie
        document.cookie = 'gengoka-auth-present=; path=/; max-age=0';
        set({ accessToken: null, refreshToken: null, user: null });
      },
    }),
    {
      name: 'gengoka-auth',
      partialize: (state) => ({
        refreshToken: state.refreshToken,
        user: state.user,
      }),
    },
  ),
);
