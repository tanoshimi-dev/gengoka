import type { Metadata } from 'next';

export const metadata: Metadata = {
  title: 'マイページ - Gengoka',
};

export default function MyPage() {
  return (
    <div className="flex flex-col items-center justify-center py-20">
      <h1 className="text-2xl font-bold text-[#1a1a2e]">マイページ</h1>
      <p className="mt-2 text-sm text-[#999999]">Phase 3 で実装予定</p>
    </div>
  );
}
