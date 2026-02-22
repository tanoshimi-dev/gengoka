'use client';

import Link from 'next/link';
import { useCategories } from '@/hooks/useCategories';
import { CATEGORY_COLORS } from '@/lib/utils/constants';

export function CategoryGrid() {
  const { data, isLoading } = useCategories();

  if (isLoading) {
    return (
      <div className="grid grid-cols-2 gap-3">
        {Array.from({ length: 4 }).map((_, i) => (
          <div key={i} className="h-32 animate-pulse rounded-xl bg-white shadow-sm" />
        ))}
      </div>
    );
  }

  if (!data || data.length === 0) return null;

  return (
    <section>
      <h2 className="mb-3 text-lg font-bold text-[#1a1a2e]">カテゴリ</h2>
      <div className="grid grid-cols-2 gap-3">
        {data.map((cat) => {
          const color = cat.color_hex || CATEGORY_COLORS[cat.name] || '#667eea';
          return (
            <Link
              key={cat.id}
              href={`/categories/${cat.id}`}
              className="flex flex-col rounded-xl bg-white p-4 shadow-sm transition-transform hover:scale-[1.02]"
            >
              <span
                className="flex h-10 w-10 items-center justify-center rounded-lg text-lg"
                style={{ backgroundColor: `${color}20`, color }}
              >
                {cat.icon_name || '📝'}
              </span>
              <p className="mt-2 text-sm font-semibold text-[#1a1a2e]">{cat.name}</p>
              <p className="mt-1 line-clamp-2 text-xs text-[#999999]">
                {cat.description || ''}
              </p>
              <span className="mt-auto pt-2 text-xs text-[#667eea]">
                {cat.challenge_count}問
              </span>
            </Link>
          );
        })}
      </div>
    </section>
  );
}
