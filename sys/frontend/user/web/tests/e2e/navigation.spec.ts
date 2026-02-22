import { test, expect } from '@playwright/test';

test.describe('Navigation', () => {
  test('login page has correct title', async ({ page }) => {
    await page.goto('/login');
    await expect(page).toHaveTitle(/ログイン.*Gengoka/);
  });

  test('register page has correct title', async ({ page }) => {
    await page.goto('/register');
    await expect(page).toHaveTitle(/新規登録.*Gengoka/);
  });

  test('unauthenticated users cannot access protected routes', async ({ page }) => {
    const protectedRoutes = ['/home', '/feed', '/mypage', '/rankings'];

    for (const route of protectedRoutes) {
      await page.goto(route);
      await expect(page).toHaveURL(/\/login/);
    }
  });
});
