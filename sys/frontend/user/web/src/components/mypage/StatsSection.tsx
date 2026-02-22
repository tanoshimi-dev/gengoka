'use client';

import {
  Flame,
  Trophy,
  CheckCircle2,
  Star,
  FileText,
  Users,
  UserPlus,
  Heart,
} from 'lucide-react';
import type { MyPageResponse } from '@/types/user';

interface StatsSectionProps {
  data: MyPageResponse;
}

const learningStats = [
  { key: 'current_streak' as const, label: '連続日数', icon: Flame, color: '#FF9500' },
  { key: 'best_streak' as const, label: 'ベスト', icon: Trophy, color: '#FFD700' },
  { key: 'completed_today' as const, label: '本日完了', icon: CheckCircle2, color: '#34C759' },
  { key: 'average_score' as const, label: '平均点', icon: Star, color: '#667eea', decimal: true },
  { key: 'total_challenges' as const, label: '累計', icon: FileText, color: '#764ba2' },
];

const socialStats = [
  { key: 'follower_count' as const, label: 'フォロワー', icon: Users, color: '#667eea' },
  { key: 'following_count' as const, label: 'フォロー中', icon: UserPlus, color: '#34C759' },
  { key: 'answer_count' as const, label: '回答数', icon: FileText, color: '#007AFF' },
  { key: 'total_likes' as const, label: '総いいね', icon: Heart, color: '#FF2D55' },
];

function StatCard({
  icon: Icon,
  color,
  value,
  label,
}: {
  icon: React.ElementType;
  color: string;
  value: string | number;
  label: string;
}) {
  return (
    <div className="flex flex-col items-center gap-1 rounded-xl bg-white p-3 shadow-sm">
      <Icon className="h-5 w-5" style={{ color }} />
      <span className="text-lg font-bold text-[#1a1a2e]">{value}</span>
      <span className="text-xs text-[#999999]">{label}</span>
    </div>
  );
}

export function StatsSection({ data }: StatsSectionProps) {
  return (
    <div className="space-y-4">
      <div>
        <h3 className="mb-2 text-sm font-semibold text-[#1a1a2e]">学習統計</h3>
        <div className="grid grid-cols-3 gap-2 sm:grid-cols-5">
          {learningStats.map((stat) => (
            <StatCard
              key={stat.key}
              icon={stat.icon}
              color={stat.color}
              value={stat.decimal ? data[stat.key].toFixed(1) : data[stat.key]}
              label={stat.label}
            />
          ))}
        </div>
      </div>

      <div>
        <h3 className="mb-2 text-sm font-semibold text-[#1a1a2e]">ソーシャル</h3>
        <div className="grid grid-cols-2 gap-2 sm:grid-cols-4">
          {socialStats.map((stat) => (
            <StatCard
              key={stat.key}
              icon={stat.icon}
              color={stat.color}
              value={data[stat.key]}
              label={stat.label}
            />
          ))}
        </div>
      </div>
    </div>
  );
}
