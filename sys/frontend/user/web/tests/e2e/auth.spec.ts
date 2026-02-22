import { test, expect } from '@playwright/test';

test.describe('Authentication flow', () => {
  test('redirects to login when not authenticated', async ({ page }) => {
    await page.goto('/home');
    await expect(page).toHaveURL(/\/login/);
  });

  test('login page renders correctly', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByText('ログイン')).toBeVisible();
    await expect(page.getByPlaceholder('メールアドレス')).toBeVisible();
    await expect(page.getByPlaceholder('パスワード')).toBeVisible();
  });

  test('register page renders correctly', async ({ page }) => {
    await page.goto('/register');
    await expect(page.getByText('新規登録')).toBeVisible();
  });

  test('shows validation errors on empty login', async ({ page }) => {
    await page.goto('/login');
    await page.getByRole('button', { name: 'ログイン' }).click();
    await expect(page.getByText('メールアドレスを入力')).toBeVisible();
  });

  test('navigates between login and register', async ({ page }) => {
    await page.goto('/login');
    await page.getByText('新規登録').click();
    await expect(page).toHaveURL(/\/register/);

    await page.getByText('ログイン').click();
    await expect(page).toHaveURL(/\/login/);
  });
});
