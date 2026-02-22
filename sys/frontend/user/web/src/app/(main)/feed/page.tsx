import type { Metadata } from 'next';
import { FeedContent } from './FeedContent';

export const metadata: Metadata = {
  title: 'タイムライン - Gengoka',
};

export default function FeedPage() {
  return <FeedContent />;
}
