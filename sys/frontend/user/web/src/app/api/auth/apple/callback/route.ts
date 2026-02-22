import { NextRequest, NextResponse } from 'next/server';

function getExternalOrigin(request: NextRequest): string {
  const proto = request.headers.get('x-forwarded-proto') || 'https';
  const host = request.headers.get('x-forwarded-host') || request.headers.get('host');
  if (host) {
    return `${proto}://${host}`;
  }
  return request.nextUrl.origin;
}

export async function POST(request: NextRequest) {
  const formData = await request.formData();
  const idToken = formData.get('id_token') as string | null;
  const state = formData.get('state') as string | null;
  const error = formData.get('error') as string | null;
  const origin = getExternalOrigin(request);

  if (error) {
    return NextResponse.redirect(new URL('/login?error=apple_denied', origin));
  }

  if (!idToken || !state) {
    return NextResponse.redirect(new URL('/login?error=apple_invalid', origin));
  }

  const callbackUrl = new URL('/apple-callback', origin);
  callbackUrl.searchParams.set('id_token', idToken);
  callbackUrl.searchParams.set('state', state);

  return NextResponse.redirect(callbackUrl);
}
