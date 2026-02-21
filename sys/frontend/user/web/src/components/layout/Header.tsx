'use client';

import Link from 'next/link';
import { useAuth } from '@/hooks/useAuth';
import { useAuthStore } from '@/stores/auth-store';
import { UserAvatar } from '@/components/common/UserAvatar';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { LogOut, Settings, User } from 'lucide-react';

export function Header() {
  const { logout, isLoggingOut } = useAuth();
  const user = useAuthStore((s) => s.user);

  return (
    <header className="sticky top-0 z-40 border-b border-[#e0e0e0] bg-white/80 backdrop-blur-sm">
      <div className="mx-auto flex h-14 max-w-7xl items-center justify-between px-4">
        <Link
          href="/home"
          className="bg-gradient-to-r from-[#667eea] to-[#764ba2] bg-clip-text text-xl font-bold text-transparent"
        >
          Gengoka
        </Link>

        {user && (
          <DropdownMenu>
            <DropdownMenuTrigger className="outline-none">
              <UserAvatar name={user.name} avatar={user.avatar} />
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end" className="w-48">
              <div className="px-2 py-1.5">
                <p className="text-sm font-medium">{user.name}</p>
              </div>
              <DropdownMenuSeparator />
              <DropdownMenuItem asChild>
                <Link href="/mypage">
                  <User className="mr-2 h-4 w-4" />
                  マイページ
                </Link>
              </DropdownMenuItem>
              <DropdownMenuItem asChild>
                <Link href="/settings">
                  <Settings className="mr-2 h-4 w-4" />
                  設定
                </Link>
              </DropdownMenuItem>
              <DropdownMenuSeparator />
              <DropdownMenuItem onClick={() => logout()} disabled={isLoggingOut}>
                <LogOut className="mr-2 h-4 w-4" />
                ログアウト
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        )}
      </div>
    </header>
  );
}
