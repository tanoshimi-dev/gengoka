import type { Metadata } from 'next';
import { RankingsContent } from './RankingsContent';

export const metadata: Metadata = {
  title: 'ランキング - Gengoka',
};

export default function RankingsPage() {
  return <RankingsContent />;
}
