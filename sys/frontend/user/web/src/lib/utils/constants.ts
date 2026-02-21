export const CATEGORY_COLORS: Record<string, string> = {
  状況説明: '#FF9500',
  要約: '#007AFF',
  感情表現: '#FF2D55',
  言い換え: '#34C759',
  説明: '#AF52DE',
};

export const NAV_ITEMS = [
  { href: '/home', label: 'ホーム', icon: 'Home' },
  { href: '/feed', label: 'タイムライン', icon: 'Newspaper' },
  { href: '/mypage', label: 'マイページ', icon: 'User' },
  { href: '/rankings', label: 'ランキング', icon: 'Trophy' },
] as const;
