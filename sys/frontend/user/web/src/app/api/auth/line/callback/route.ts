import { NextRequest, NextResponse } from 'next/server';

export async function GET(request: NextRequest) {
  const { searchParams } = new URL(request.url);
  const code = searchParams.get('code');
  const state = searchParams.get('state');
  const error = searchParams.get('error');

  if (error) {
    return NextResponse.redirect(new URL('/login?error=line_denied', request.url));
  }

  if (!code || !state) {
    return NextResponse.redirect(new URL('/login?error=line_invalid', request.url));
  }

  const channelId = process.env.NEXT_PUBLIC_LINE_CHANNEL_ID;
  const channelSecret = process.env.LINE_CHANNEL_SECRET;

  if (!channelId || !channelSecret) {
    return NextResponse.redirect(new URL('/login?error=line_config', request.url));
  }

  const redirectUri = `${request.nextUrl.origin}/api/auth/line/callback`;

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
      return NextResponse.redirect(new URL('/login?error=line_token', request.url));
    }

    const tokenData = await tokenResponse.json();
    const accessToken = tokenData.access_token as string;

    const callbackUrl = new URL('/line-callback', request.url);
    callbackUrl.searchParams.set('access_token', accessToken);
    callbackUrl.searchParams.set('state', state);

    return NextResponse.redirect(callbackUrl);
  } catch {
    return NextResponse.redirect(new URL('/login?error=line_failed', request.url));
  }
}
