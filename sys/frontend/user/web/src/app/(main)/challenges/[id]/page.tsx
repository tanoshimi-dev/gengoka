import type { Metadata } from 'next';
import { ChallengeContent } from './ChallengeContent';

export const metadata: Metadata = {
  title: 'チャレンジ - Gengoka',
};

export default function ChallengePage() {
  return <ChallengeContent />;
}
