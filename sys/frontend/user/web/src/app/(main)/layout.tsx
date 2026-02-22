import { Header } from '@/components/layout/Header';
import { Sidebar } from '@/components/layout/Sidebar';
import { BottomNav } from '@/components/layout/BottomNav';

export default function MainLayout({ children }: { children: React.ReactNode }) {
  return (
    <div className="min-h-screen bg-[#F8F9FA]">
      <Header />
      <div className="mx-auto flex w-full max-w-7xl overflow-hidden">
        <Sidebar />
        <main className="min-w-0 flex-1 overflow-x-hidden px-4 py-6 pb-20 lg:px-6 lg:pb-6">
          {children}
        </main>
      </div>
      <BottomNav />
    </div>
  );
}
