import type { Metadata } from 'next';
import { CategoryContent } from './CategoryContent';

export const metadata: Metadata = {
  title: 'カテゴリ - Gengoka',
};

export default function CategoryPage() {
  return <CategoryContent />;
}
