import type { Metadata } from 'next';
import { Noto_Sans_JP } from 'next/font/google';
import './globals.css';
import { Providers } from './providers';
import { AuthProvider } from '@/components/auth/AuthProvider';
import { Toaster } from '@/components/ui/sonner';

const notoSansJP = Noto_Sans_JP({
  variable: '--font-noto-sans-jp',
  subsets: ['latin'],
  display: 'swap',
});

export const metadata: Metadata = {
  title: 'Gengoka - 言語化トレーニング',
  description: '言語化力を鍛えるトレーニングプラットフォーム',
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="ja">
      <body className={`${notoSansJP.variable} font-sans antialiased`}>
        <Providers>
          <AuthProvider>
            {children}
            <Toaster />
          </AuthProvider>
        </Providers>
      </body>
    </html>
  );
}
