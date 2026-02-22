import { NextRequest, NextResponse } from 'next/server';

export async function POST(request: NextRequest) {
  const formData = await request.formData();
  const idToken = formData.get('id_token') as string | null;
  const state = formData.get('state') as string | null;
  const error = formData.get('error') as string | null;

  if (error) {
    return NextResponse.redirect(new URL('/login?error=apple_denied', request.url));
  }

  if (!idToken || !state) {
    return NextResponse.redirect(new URL('/login?error=apple_invalid', request.url));
  }

  const callbackUrl = new URL('/apple-callback', request.url);
  callbackUrl.searchParams.set('id_token', idToken);
  callbackUrl.searchParams.set('state', state);

  return NextResponse.redirect(callbackUrl);
}
