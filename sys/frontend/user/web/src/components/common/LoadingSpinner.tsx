import { Loader2 } from 'lucide-react';

interface LoadingSpinnerProps {
  message?: string;
}

export function LoadingSpinner({ message = '読み込み中...' }: LoadingSpinnerProps) {
  return (
    <div className="flex flex-col items-center justify-center gap-3 py-12">
      <Loader2 className="h-8 w-8 animate-spin text-[#667eea]" />
      <p className="text-sm text-[#999999]">{message}</p>
    </div>
  );
}
