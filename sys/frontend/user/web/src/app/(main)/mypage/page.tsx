import type { Metadata } from 'next';
import { MyPageContent } from './MyPageContent';

export const metadata: Metadata = {
  title: 'マイページ - Gengoka',
};

export default function MyPage() {
  return <MyPageContent />;
}
