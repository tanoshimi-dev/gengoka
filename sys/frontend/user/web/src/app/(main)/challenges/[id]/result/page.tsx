import type { Metadata } from 'next';
import { Suspense } from 'react';
import { ResultContent } from './ResultContent';
import { LoadingSpinner } from '@/components/common/LoadingSpinner';

export const metadata: Metadata = {
  title: '結果 - Gengoka',
};

export default function ResultPage() {
  return (
    <Suspense fallback={<LoadingSpinner />}>
      <ResultContent />
    </Suspense>
  );
}
