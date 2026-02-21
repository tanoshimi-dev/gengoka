import { AlertCircle } from 'lucide-react';
import { GradientButton } from './GradientButton';

interface ErrorMessageProps {
  message?: string;
  onRetry?: () => void;
}

export function ErrorMessage({
  message = 'エラーが発生しました',
  onRetry,
}: ErrorMessageProps) {
  return (
    <div className="flex flex-col items-center justify-center gap-3 py-12">
      <AlertCircle className="h-8 w-8 text-[#E53935]" />
      <p className="text-sm text-[#666666]">{message}</p>
      {onRetry && (
        <GradientButton variant="secondary" onClick={onRetry}>
          再試行
        </GradientButton>
      )}
    </div>
  );
}
