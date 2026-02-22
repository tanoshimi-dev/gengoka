'use client';

import { Flame, CheckCircle2, Star, FileText } from 'lucide-react';
import { useUserStats } from '@/hooks/useUserStats';

const stats = [
  { key: 'current_streak', label: '連続', icon: Flame, color: '#FF9500' },
  { key: 'completed_today', label: '本日', icon: CheckCircle2, color: '#34C759' },
  { key: 'average_score', label: '平均', icon: Star, color: '#FFD700' },
  { key: 'total_challenges', label: '総数', icon: FileText, color: '#667eea' },
] as const;

export function StatsBar() {
  const { data, isLoading } = useUserStats();

  if (isLoading) {
    return (
      <div className="grid grid-cols-4 gap-3">
        {Array.from({ length: 4 }).map((_, i) => (
          <div key={i} className="h-20 animate-pulse rounded-xl bg-white shadow-sm" />
        ))}
      </div>
    );
  }

  if (!data) return null;

  return (
    <div className="grid grid-cols-4 gap-3">
      {stats.map(({ key, label, icon: Icon, color }) => (
        <div
          key={key}
          className="flex flex-col items-center gap-1 rounded-xl bg-white p-3 shadow-sm"
        >
          <Icon className="h-5 w-5" style={{ color }} />
          <span className="text-lg font-bold text-[#1a1a2e]">
            {key === 'average_score'
              ? data[key].toFixed(1)
              : data[key]}
          </span>
          <span className="text-xs text-[#999999]">{label}</span>
        </div>
      ))}
    </div>
  );
}
