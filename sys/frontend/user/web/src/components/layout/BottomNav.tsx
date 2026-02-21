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

export function BottomNav() {
  const pathname = usePathname();

  return (
    <nav className="fixed bottom-0 left-0 right-0 z-40 border-t border-[#e0e0e0] bg-white lg:hidden">
      <div className="flex h-16 items-center justify-around">
        {NAV_ITEMS.map((item) => {
          const Icon = iconMap[item.icon];
          const isActive = pathname.startsWith(item.href);
          return (
            <Link
              key={item.href}
              href={item.href}
              className={cn(
                'flex flex-col items-center gap-1 px-3 py-1',
                isActive ? 'text-[#667eea]' : 'text-[#999999]',
              )}
            >
              <Icon className="h-5 w-5" />
              <span className="text-[10px] font-medium">{item.label}</span>
            </Link>
          );
        })}
      </div>
    </nav>
  );
}
