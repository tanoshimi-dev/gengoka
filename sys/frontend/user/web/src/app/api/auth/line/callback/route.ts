import { NextRequest, NextResponse } from 'next/server';

function getExternalOrigin(request: NextRequest): string {
  const proto = request.headers.get('x-forwarded-proto') || 'https';
  const host = request.headers.get('x-forwarded-host') || request.headers.get('host');
  if (host) {
    return `${proto}://${host}`;
  }
  return request.nextUrl.origin;
}

export async function GET(request: NextRequest) {
  const { searchParams } = new URL(request.url);
  const code = searchParams.get('code');
  const state = searchParams.get('state');
  const error = searchParams.get('error');
  const origin = getExternalOrigin(request);

  if (error) {
    return NextResponse.redirect(new URL('/login?error=line_denied', origin));
  }

  if (!code || !state) {
    return NextResponse.redirect(new URL('/login?error=line_invalid', origin));
  }

  const channelId = process.env.NEXT_PUBLIC_LINE_CHANNEL_ID;
  const channelSecret = process.env.LINE_CHANNEL_SECRET;

  if (!channelId || !channelSecret) {
    return NextResponse.redirect(new URL('/login?error=line_config', origin));
  }

  const redirectUri = `${origin}/api/auth/line/callback`;

  try {
    const tokenResponse = await fetch('https://api.line.me/oauth2/v2.1/token', {
      method: 'POST',
      headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
      body: new URLSearchParams({
        grant_type: 'authorization_code',
        code,
        redirect_uri: redirectUri,
        client_id: channelId,
        client_secret: channelSecret,
      }),
    });

    if (!tokenResponse.ok) {
      return NextResponse.redirect(new URL('/login?error=line_token', origin));
    }

    const tokenData = await tokenResponse.json();
    const accessToken = tokenData.access_token as string;

    const callbackUrl = new URL('/line-callback', origin);
    callbackUrl.searchParams.set('access_token', accessToken);
    callbackUrl.searchParams.set('state', state);

    return NextResponse.redirect(callbackUrl);
  } catch {
    return NextResponse.redirect(new URL('/login?error=line_failed', origin));
  }
}
