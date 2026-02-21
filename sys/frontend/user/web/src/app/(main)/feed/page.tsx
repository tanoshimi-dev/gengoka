import type { Metadata } from 'next';

export const metadata: Metadata = {
  title: 'タイムライン - Gengoka',
};

export default function FeedPage() {
  return (
    <div className="flex flex-col items-center justify-center py-20">
      <h1 className="text-2xl font-bold text-[#1a1a2e]">タイムライン</h1>
      <p className="mt-2 text-sm text-[#999999]">Phase 3 で実装予定</p>
    </div>
  );
}
