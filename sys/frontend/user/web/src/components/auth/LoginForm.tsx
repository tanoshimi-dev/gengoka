'use client';

import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { loginSchema, type LoginFormValues } from '@/lib/utils/validation';
import { useAuth } from '@/hooks/useAuth';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { GradientButton } from '@/components/common/GradientButton';
import { SocialLoginButtons } from './SocialLoginButtons';
import { Separator } from '@/components/ui/separator';
import Link from 'next/link';

export function LoginForm() {
  const { login, isLoggingIn, loginError } = useAuth();
  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<LoginFormValues>({
    resolver: zodResolver(loginSchema),
  });

  const onSubmit = async (data: LoginFormValues) => {
    try {
      await login({ email: data.email, password: data.password });
    } catch {
      // error handled by mutation
    }
  };

  return (
    <div className="w-full max-w-md space-y-6">
      <div className="text-center">
        <h1 className="text-2xl font-bold text-[#1a1a2e]">ログイン</h1>
        <p className="mt-2 text-sm text-[#666666]">アカウントにログインしてください</p>
      </div>

      <SocialLoginButtons />

      <div className="flex items-center gap-4">
        <Separator className="flex-1" />
        <span className="text-xs text-[#999999]">または</span>
        <Separator className="flex-1" />
      </div>

      <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
        <div className="space-y-2">
          <Label htmlFor="email">メールアドレス</Label>
          <Input
            id="email"
            type="email"
            placeholder="example@email.com"
            {...register('email')}
          />
          {errors.email && <p className="text-xs text-red-500">{errors.email.message}</p>}
        </div>

        <div className="space-y-2">
          <Label htmlFor="password">パスワード</Label>
          <Input id="password" type="password" placeholder="••••••••" {...register('password')} />
          {errors.password && <p className="text-xs text-red-500">{errors.password.message}</p>}
        </div>

        {loginError && (
          <p className="text-xs text-red-500">
            {loginError.message === 'Request failed: 401'
              ? 'メールアドレスまたはパスワードが正しくありません'
              : 'ログインに失敗しました。もう一度お試しください'}
          </p>
        )}

        <GradientButton type="submit" loading={isLoggingIn} className="w-full">
          ログイン
        </GradientButton>
      </form>

      <p className="text-center text-sm text-[#666666]">
        アカウントをお持ちでないですか？{' '}
        <Link href="/register" className="font-semibold text-[#667eea] hover:underline">
          新規登録
        </Link>
      </p>
    </div>
  );
}
