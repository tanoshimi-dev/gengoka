'use client';

import { forwardRef } from 'react';
import { cn } from '@/lib/utils';
import { Loader2 } from 'lucide-react';

interface GradientButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'secondary';
  loading?: boolean;
}

export const GradientButton = forwardRef<HTMLButtonElement, GradientButtonProps>(
  ({ className, variant = 'primary', loading, disabled, children, ...props }, ref) => {
    return (
      <button
        ref={ref}
        disabled={disabled || loading}
        className={cn(
          'inline-flex items-center justify-center rounded-lg px-6 py-3 text-sm font-semibold transition-all duration-200 disabled:opacity-50',
          variant === 'primary' &&
            'bg-gradient-to-r from-[#667eea] to-[#764ba2] text-white shadow-md hover:shadow-lg hover:brightness-110',
          variant === 'secondary' &&
            'border border-[#667eea] bg-white text-[#667eea] hover:bg-[#667eea]/5',
          className,
        )}
        {...props}
      >
        {loading && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
        {children}
      </button>
    );
  },
);
GradientButton.displayName = 'GradientButton';
