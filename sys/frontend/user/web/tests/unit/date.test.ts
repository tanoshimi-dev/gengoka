import { describe, it, expect, vi, afterEach } from 'vitest';
import { formatRelativeTime } from '@/lib/utils/date';

describe('formatRelativeTime', () => {
  afterEach(() => {
    vi.useRealTimers();
  });

  it('returns "たった今" for less than 60 seconds ago', () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date('2025-01-01T12:00:30Z'));
    expect(formatRelativeTime('2025-01-01T12:00:00Z')).toBe('たった今');
  });

  it('returns minutes for less than 60 minutes ago', () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date('2025-01-01T12:05:00Z'));
    expect(formatRelativeTime('2025-01-01T12:00:00Z')).toBe('5分前');
  });

  it('returns hours for less than 24 hours ago', () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date('2025-01-01T15:00:00Z'));
    expect(formatRelativeTime('2025-01-01T12:00:00Z')).toBe('3時間前');
  });

  it('returns days for less than 7 days ago', () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date('2025-01-04T12:00:00Z'));
    expect(formatRelativeTime('2025-01-01T12:00:00Z')).toBe('3日前');
  });

  it('returns formatted date for 7+ days ago', () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date('2025-01-15T12:00:00Z'));
    const result = formatRelativeTime('2025-01-01T12:00:00Z');
    // ja-JP locale date format
    expect(result).toContain('2025');
    expect(result).toContain('1');
  });

  it('handles edge case at exactly 60 seconds', () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date('2025-01-01T12:01:00Z'));
    expect(formatRelativeTime('2025-01-01T12:00:00Z')).toBe('1分前');
  });

  it('handles edge case at exactly 60 minutes', () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date('2025-01-01T13:00:00Z'));
    expect(formatRelativeTime('2025-01-01T12:00:00Z')).toBe('1時間前');
  });

  it('handles edge case at exactly 24 hours', () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date('2025-01-02T12:00:00Z'));
    expect(formatRelativeTime('2025-01-01T12:00:00Z')).toBe('1日前');
  });
});
