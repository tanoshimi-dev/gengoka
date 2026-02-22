'use client';

import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import { GradientButton } from '@/components/common/GradientButton';
import { useUpdateProfile } from '@/hooks/useMyPage';

const profileSchema = z.object({
  name: z.string().min(1, '名前を入力してください').max(50, '名前は50文字以内です'),
  bio: z.string().max(200, '自己紹介は200文字以内です'),
});

type ProfileFormData = z.infer<typeof profileSchema>;

interface ProfileEditFormProps {
  defaultValues: { name: string; bio: string };
}

export function ProfileEditForm({ defaultValues }: ProfileEditFormProps) {
  const updateProfile = useUpdateProfile();
  const {
    register,
    handleSubmit,
    formState: { errors, isDirty },
  } = useForm<ProfileFormData>({
    resolver: zodResolver(profileSchema),
    defaultValues,
  });

  const onSubmit = (data: ProfileFormData) => {
    updateProfile.mutate(data);
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
      <h3 className="text-sm font-semibold text-[#1a1a2e]">プロフィール編集</h3>

      <div>
        <label className="mb-1 block text-sm font-medium text-[#333333]">名前</label>
        <input
          {...register('name')}
          className="w-full rounded-lg border border-[#e0e0e0] bg-white px-4 py-2.5 text-sm outline-none focus:border-[#667eea]"
        />
        {errors.name && (
          <p className="mt-1 text-xs text-[#E53935]">{errors.name.message}</p>
        )}
      </div>

      <div>
        <label className="mb-1 block text-sm font-medium text-[#333333]">
          自己紹介
        </label>
        <textarea
          {...register('bio')}
          rows={3}
          className="w-full resize-none rounded-lg border border-[#e0e0e0] bg-white px-4 py-2.5 text-sm outline-none focus:border-[#667eea]"
        />
        {errors.bio && (
          <p className="mt-1 text-xs text-[#E53935]">{errors.bio.message}</p>
        )}
      </div>

      <GradientButton
        type="submit"
        loading={updateProfile.isPending}
        disabled={!isDirty}
        className="w-full"
      >
        保存
      </GradientButton>
    </form>
  );
}
