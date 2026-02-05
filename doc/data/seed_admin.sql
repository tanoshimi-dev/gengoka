-- Seed Admin User for Gengoka Admin Panel
--
-- RECOMMENDED: Use the CLI command instead:
--   ./gengoka-backend create-admin
--
-- This SQL is for reference only. To use it, you must generate
-- a proper argon2id hash for your password.
--
-- Example using the CLI:
--   docker run -it --rm -e DATABASE_URL=... gengoka-backend ./gengoka-backend create-admin

-- Uncomment and modify after generating proper password hash:
-- INSERT INTO admin_users (id, email, password_hash, name, role, status, created_at, updated_at)
-- VALUES (
--     gen_random_uuid(),
--     'admin@gengoka.com',
--     '<argon2id-password-hash-here>',
--     'Administrator',
--     'super_admin',
--     'active',
--     NOW(),
--     NOW()
-- )
-- ON CONFLICT (email) DO NOTHING;

-- Initial system configuration
INSERT INTO system_config (key, value, description, updated_at)
VALUES
    ('gemini', '{"model": "gemini-1.5-flash", "max_tokens": 1024, "temperature": 0.7}', 'Gemini AI configuration', NOW()),
    ('scheduler', '{"enabled": false, "daily_count": 5, "cron_schedule": "0 0 9 * * *"}', 'Challenge scheduler configuration', NOW())
ON CONFLICT (key) DO NOTHING;
