import { describe, it, expect } from 'vitest';
import { CATEGORY_COLORS, NAV_ITEMS } from '@/lib/utils/constants';

describe('CATEGORY_COLORS', () => {
  it('has 5 category colors', () => {
    expect(Object.keys(CATEGORY_COLORS)).toHaveLength(5);
  });

  it('maps correct colors to categories', () => {
    expect(CATEGORY_COLORS['状況説明']).toBe('#FF9500');
    expect(CATEGORY_COLORS['要約']).toBe('#007AFF');
    expect(CATEGORY_COLORS['感情表現']).toBe('#FF2D55');
    expect(CATEGORY_COLORS['言い換え']).toBe('#34C759');
    expect(CATEGORY_COLORS['説明']).toBe('#AF52DE');
  });

  it('returns undefined for unknown category', () => {
    expect(CATEGORY_COLORS['未知のカテゴリ']).toBeUndefined();
  });
});

describe('NAV_ITEMS', () => {
  it('has 4 navigation items', () => {
    expect(NAV_ITEMS).toHaveLength(4);
  });

  it('has correct paths', () => {
    const paths = NAV_ITEMS.map((item) => item.href);
    expect(paths).toEqual(['/home', '/feed', '/mypage', '/rankings']);
  });

  it('has correct labels', () => {
    const labels = NAV_ITEMS.map((item) => item.label);
    expect(labels).toEqual(['ホーム', 'タイムライン', 'マイページ', 'ランキング']);
  });

  it('has correct icons', () => {
    const icons = NAV_ITEMS.map((item) => item.icon);
    expect(icons).toEqual(['Home', 'Newspaper', 'User', 'Trophy']);
  });
});
