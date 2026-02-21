import { useAuthStore } from '@/stores/auth-store';
import { ApiError, type ApiResponse, type PaginationInfo } from './types';
import { AUTH } from './endpoints';

const API_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080/api/v1';

async function refreshAccessToken(): Promise<string | null> {
  const { refreshToken, setTokens, clearAuth } = useAuthStore.getState();
  if (!refreshToken) return null;

  try {
    const res = await fetch(`${API_URL}${AUTH.REFRESH}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ refresh_token: refreshToken }),
    });

    if (!res.ok) {
      clearAuth();
      return null;
    }

    const json: ApiResponse<{ access_token: string; refresh_token: string; expires_in: number }> =
      await res.json();
    if (json.success && json.data) {
      setTokens(json.data.access_token, json.data.refresh_token, useAuthStore.getState().user);
      return json.data.access_token;
    }

    clearAuth();
    return null;
  } catch {
    clearAuth();
    return null;
  }
}

interface RequestOptions extends Omit<RequestInit, 'body'> {
  body?: unknown;
  params?: Record<string, string | number | undefined>;
  noAuth?: boolean;
}

export async function apiClient<T>(endpoint: string, options: RequestOptions = {}): Promise<T> {
  const { body, params, noAuth, ...init } = options;

  let url = `${API_URL}${endpoint}`;
  if (params) {
    const searchParams = new URLSearchParams();
    for (const [key, value] of Object.entries(params)) {
      if (value !== undefined) searchParams.set(key, String(value));
    }
    const qs = searchParams.toString();
    if (qs) url += `?${qs}`;
  }

  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
    ...(init.headers as Record<string, string>),
  };

  if (!noAuth) {
    const token = useAuthStore.getState().accessToken;
    if (token) headers['Authorization'] = `Bearer ${token}`;
  }

  let res = await fetch(url, {
    ...init,
    headers,
    body: body ? JSON.stringify(body) : undefined,
  });

  // Auto-refresh on 401
  if (res.status === 401 && !noAuth) {
    const newToken = await refreshAccessToken();
    if (newToken) {
      headers['Authorization'] = `Bearer ${newToken}`;
      res = await fetch(url, {
        ...init,
        headers,
        body: body ? JSON.stringify(body) : undefined,
      });
    }
  }

  if (!res.ok) {
    const errorBody = await res.json().catch(() => null);
    throw new ApiError(res.status, errorBody?.error || `Request failed: ${res.status}`);
  }

  // Handle 204 No Content
  if (res.status === 204) return undefined as T;

  const json: ApiResponse<T> = await res.json();
  if (!json.success) {
    throw new ApiError(res.status, json.error || 'Unknown error');
  }

  return json.data as T;
}

export async function apiClientPaginated<T>(
  endpoint: string,
  options: RequestOptions = {},
): Promise<{ data: T; pagination: PaginationInfo }> {
  const { body, params, noAuth, ...init } = options;

  let url = `${API_URL}${endpoint}`;
  if (params) {
    const searchParams = new URLSearchParams();
    for (const [key, value] of Object.entries(params)) {
      if (value !== undefined) searchParams.set(key, String(value));
    }
    const qs = searchParams.toString();
    if (qs) url += `?${qs}`;
  }

  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
    ...(init.headers as Record<string, string>),
  };

  if (!noAuth) {
    const token = useAuthStore.getState().accessToken;
    if (token) headers['Authorization'] = `Bearer ${token}`;
  }

  let res = await fetch(url, {
    ...init,
    headers,
    body: body ? JSON.stringify(body) : undefined,
  });

  if (res.status === 401 && !noAuth) {
    const newToken = await refreshAccessToken();
    if (newToken) {
      headers['Authorization'] = `Bearer ${newToken}`;
      res = await fetch(url, {
        ...init,
        headers,
        body: body ? JSON.stringify(body) : undefined,
      });
    }
  }

  if (!res.ok) {
    const errorBody = await res.json().catch(() => null);
    throw new ApiError(res.status, errorBody?.error || `Request failed: ${res.status}`);
  }

  const json: ApiResponse<T> = await res.json();
  if (!json.success) {
    throw new ApiError(res.status, json.error || 'Unknown error');
  }

  return {
    data: json.data as T,
    pagination: json.pagination!,
  };
}
