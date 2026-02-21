'use client';

import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { Home, Newspaper, User, Trophy } from 'lucide-react';
import { cn } from '@/lib/utils';
import { NAV_ITEMS } from '@/lib/utils/constants';

const iconMap = {
  Home,
  Newspaper,
  User,
  Trophy,
} as const;

export function Sidebar() {
  const pathname = usePathname();

  return (
    <aside className="sticky top-14 hidden h-[calc(100vh-3.5rem)] w-56 shrink-0 border-r border-[#e0e0e0] bg-white lg:block">
      <nav className="flex flex-col gap-1 p-3">
        {NAV_ITEMS.map((item) => {
          const Icon = iconMap[item.icon];
          const isActive = pathname.startsWith(item.href);
          return (
            <Link
              key={item.href}
              href={item.href}
              className={cn(
                'flex items-center gap-3 rounded-lg px-3 py-2.5 text-sm font-medium transition-colors',
                isActive
                  ? 'bg-gradient-to-r from-[#667eea]/10 to-[#764ba2]/10 text-[#667eea]'
                  : 'text-[#666666] hover:bg-[#F8F9FA] hover:text-[#1a1a2e]',
              )}
            >
              <Icon className="h-5 w-5" />
              {item.label}
            </Link>
          );
        })}
      </nav>
    </aside>
  );
}
