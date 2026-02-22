'use client';

import { cn } from '@/lib/utils';

interface FilterTabsProps {
  filter: 'all' | 'following';
  onChange: (filter: 'all' | 'following') => void;
}

const tabs = [
  { value: 'all' as const, label: 'すべて' },
  { value: 'following' as const, label: 'フォロー中' },
];

export function FilterTabs({ filter, onChange }: FilterTabsProps) {
  return (
    <div className="flex gap-1 rounded-lg bg-white p-1 shadow-sm">
      {tabs.map((tab) => (
        <button
          key={tab.value}
          onClick={() => onChange(tab.value)}
          className={cn(
            'relative flex-1 rounded-md px-4 py-2 text-sm font-medium transition-all duration-200',
            filter === tab.value
              ? 'text-white'
              : 'text-[#999999] hover:text-[#1a1a2e]',
          )}
        >
          {filter === tab.value && (
            <span className="absolute inset-0 rounded-md bg-gradient-to-r from-[#667eea] to-[#764ba2]" />
          )}
          <span className="relative">{tab.label}</span>
        </button>
      ))}
    </div>
  );
}
