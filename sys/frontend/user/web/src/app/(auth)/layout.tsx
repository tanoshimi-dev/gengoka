export default function AuthLayout({ children }: { children: React.ReactNode }) {
  return (
    <div className="flex min-h-screen items-center justify-center bg-gradient-to-br from-[#667eea]/10 via-white to-[#764ba2]/10 px-4">
      {children}
    </div>
  );
}
