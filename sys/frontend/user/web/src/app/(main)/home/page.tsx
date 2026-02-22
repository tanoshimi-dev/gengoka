import type { Metadata } from 'next';
import { HomeContent } from './HomeContent';

export const metadata: Metadata = {
  title: 'ホーム - Gengoka',
};

export default function HomePage() {
  return <HomeContent />;
}
