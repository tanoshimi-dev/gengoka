import type { UserSummary } from './user';

export interface AuthTokens {
  access_token: string;
  refresh_token: string;
  expires_in: number;
  user: UserSummary;
}

export interface LoginRequest {
  email: string;
  password: string;
  device_info?: string;
}

export interface SignupRequest {
  email: string;
  password: string;
  name: string;
  device_info?: string;
}

export interface RefreshRequest {
  refresh_token: string;
}

export interface LogoutRequest {
  refresh_token?: string;
}

export interface SocialLoginRequest {
  provider: string;
  id_token?: string;
  access_token?: string;
  authorization_code?: string;
  nonce?: string;
  device_info?: string;
}
