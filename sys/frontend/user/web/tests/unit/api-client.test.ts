import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { apiClient, apiClientPaginated } from '@/lib/api/client';
import { ApiError } from '@/lib/api/types';

// Mock zustand store
vi.mock('@/stores/auth-store', () => ({
  useAuthStore: {
    getState: vi.fn(() => ({
      accessToken: 'test-token',
      refreshToken: 'test-refresh',
      user: null,
      setTokens: vi.fn(),
      clearAuth: vi.fn(),
    })),
  },
}));

describe('apiClient', () => {
  beforeEach(() => {
    vi.stubGlobal('fetch', vi.fn());
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('makes GET request and returns data', async () => {
    const mockData = { id: '1', name: 'Test' };
    vi.mocked(fetch).mockResolvedValueOnce({
      ok: true,
      status: 200,
      json: () => Promise.resolve({ success: true, data: mockData }),
    } as Response);

    const result = await apiClient('/test');
    expect(result).toEqual(mockData);
    expect(fetch).toHaveBeenCalledOnce();
  });

  it('includes Authorization header when token exists', async () => {
    vi.mocked(fetch).mockResolvedValueOnce({
      ok: true,
      status: 200,
      json: () => Promise.resolve({ success: true, data: {} }),
    } as Response);

    await apiClient('/test');
    const [, options] = vi.mocked(fetch).mock.calls[0];
    expect((options?.headers as Record<string, string>)['Authorization']).toBe(
      'Bearer test-token',
    );
  });

  it('skips auth header when noAuth is true', async () => {
    vi.mocked(fetch).mockResolvedValueOnce({
      ok: true,
      status: 200,
      json: () => Promise.resolve({ success: true, data: {} }),
    } as Response);

    await apiClient('/test', { noAuth: true });
    const [, options] = vi.mocked(fetch).mock.calls[0];
    expect((options?.headers as Record<string, string>)['Authorization']).toBeUndefined();
  });

  it('adds query params to URL', async () => {
    vi.mocked(fetch).mockResolvedValueOnce({
      ok: true,
      status: 200,
      json: () => Promise.resolve({ success: true, data: {} }),
    } as Response);

    await apiClient('/test', { params: { page: 1, filter: 'all' } });
    const [url] = vi.mocked(fetch).mock.calls[0];
    expect(url).toContain('page=1');
    expect(url).toContain('filter=all');
  });

  it('skips undefined params', async () => {
    vi.mocked(fetch).mockResolvedValueOnce({
      ok: true,
      status: 200,
      json: () => Promise.resolve({ success: true, data: {} }),
    } as Response);

    await apiClient('/test', { params: { page: 1, filter: undefined } });
    const [url] = vi.mocked(fetch).mock.calls[0];
    expect(url).toContain('page=1');
    expect(url).not.toContain('filter');
  });

  it('sends JSON body for POST requests', async () => {
    vi.mocked(fetch).mockResolvedValueOnce({
      ok: true,
      status: 200,
      json: () => Promise.resolve({ success: true, data: {} }),
    } as Response);

    await apiClient('/test', { method: 'POST', body: { name: 'Test' } });
    const [, options] = vi.mocked(fetch).mock.calls[0];
    expect(options?.body).toBe(JSON.stringify({ name: 'Test' }));
    expect(options?.method).toBe('POST');
  });

  it('throws ApiError on HTTP error', async () => {
    vi.mocked(fetch).mockResolvedValue({
      ok: false,
      status: 404,
      json: () => Promise.resolve({ error: 'Not found' }),
    } as Response);

    await expect(apiClient('/test')).rejects.toThrow(ApiError);
  });

  it('includes status code in ApiError', async () => {
    vi.mocked(fetch).mockResolvedValue({
      ok: false,
      status: 404,
      json: () => Promise.resolve({ error: 'Not found' }),
    } as Response);

    try {
      await apiClient('/test');
    } catch (e) {
      expect(e).toBeInstanceOf(ApiError);
      expect((e as ApiError).statusCode).toBe(404);
    }
  });

  it('throws ApiError when success is false', async () => {
    vi.mocked(fetch).mockResolvedValueOnce({
      ok: true,
      status: 200,
      json: () => Promise.resolve({ success: false, error: 'Bad request' }),
    } as Response);

    await expect(apiClient('/test')).rejects.toThrow('Bad request');
  });

  it('handles 204 No Content', async () => {
    vi.mocked(fetch).mockResolvedValueOnce({
      ok: true,
      status: 204,
    } as Response);

    const result = await apiClient('/test', { method: 'DELETE' });
    expect(result).toBeUndefined();
  });
});

describe('apiClientPaginated', () => {
  beforeEach(() => {
    vi.stubGlobal('fetch', vi.fn());
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('returns data with pagination info', async () => {
    const mockData = [{ id: '1' }];
    const mockPagination = {
      page: 1,
      page_size: 10,
      total: 1,
      total_pages: 1,
      has_more: false,
    };

    vi.mocked(fetch).mockResolvedValueOnce({
      ok: true,
      status: 200,
      json: () =>
        Promise.resolve({
          success: true,
          data: mockData,
          pagination: mockPagination,
        }),
    } as Response);

    const result = await apiClientPaginated('/test');
    expect(result.data).toEqual(mockData);
    expect(result.pagination).toEqual(mockPagination);
  });

  it('throws on HTTP error', async () => {
    vi.mocked(fetch).mockResolvedValueOnce({
      ok: false,
      status: 500,
      json: () => Promise.resolve({ error: 'Server error' }),
    } as Response);

    await expect(apiClientPaginated('/test')).rejects.toThrow(ApiError);
  });
});
