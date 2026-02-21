import type { Metadata } from 'next';

export const metadata: Metadata = {
  title: 'ホーム - Gengoka',
};

export default function HomePage() {
  return (
    <div className="flex flex-col items-center justify-center py-20">
      <h1 className="text-2xl font-bold text-[#1a1a2e]">ホーム</h1>
      <p className="mt-2 text-sm text-[#999999]">Phase 2 で実装予定</p>
    </div>
  );
}
