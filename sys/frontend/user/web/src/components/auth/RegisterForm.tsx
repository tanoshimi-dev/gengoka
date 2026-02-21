'use client';

import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { registerSchema, type RegisterFormValues } from '@/lib/utils/validation';
import { useAuth } from '@/hooks/useAuth';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { GradientButton } from '@/components/common/GradientButton';
import { SocialLoginButtons } from './SocialLoginButtons';
import { Separator } from '@/components/ui/separator';
import Link from 'next/link';

export function RegisterForm() {
  const { register: registerUser, isRegistering, registerError } = useAuth();
  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<RegisterFormValues>({
    resolver: zodResolver(registerSchema),
  });

  const onSubmit = async (data: RegisterFormValues) => {
    try {
      await registerUser({ name: data.name, email: data.email, password: data.password });
    } catch {
      // error handled by mutation
    }
  };

  return (
    <div className="w-full max-w-md space-y-6">
      <div className="text-center">
        <h1 className="text-2xl font-bold text-[#1a1a2e]">新規登録</h1>
        <p className="mt-2 text-sm text-[#666666]">アカウントを作成してください</p>
      </div>

      <SocialLoginButtons />

      <div className="flex items-center gap-4">
        <Separator className="flex-1" />
        <span className="text-xs text-[#999999]">または</span>
        <Separator className="flex-1" />
      </div>

      <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
        <div className="space-y-2">
          <Label htmlFor="name">名前</Label>
          <Input id="name" type="text" placeholder="表示名" {...register('name')} />
          {errors.name && <p className="text-xs text-red-500">{errors.name.message}</p>}
        </div>

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
          <Input
            id="password"
            type="password"
            placeholder="8文字以上"
            {...register('password')}
          />
          {errors.password && <p className="text-xs text-red-500">{errors.password.message}</p>}
        </div>

        <div className="space-y-2">
          <Label htmlFor="confirmPassword">パスワード（確認）</Label>
          <Input
            id="confirmPassword"
            type="password"
            placeholder="パスワードを再入力"
            {...register('confirmPassword')}
          />
          {errors.confirmPassword && (
            <p className="text-xs text-red-500">{errors.confirmPassword.message}</p>
          )}
        </div>

        {registerError && (
          <p className="text-xs text-red-500">
            {registerError.message === 'Request failed: 409'
              ? 'このメールアドレスは既に登録されています'
              : '登録に失敗しました。もう一度お試しください'}
          </p>
        )}

        <GradientButton type="submit" loading={isRegistering} className="w-full">
          アカウントを作成
        </GradientButton>
      </form>

      <p className="text-center text-sm text-[#666666]">
        既にアカウントをお持ちですか？{' '}
        <Link href="/login" className="font-semibold text-[#667eea] hover:underline">
          ログイン
        </Link>
      </p>
    </div>
  );
}
