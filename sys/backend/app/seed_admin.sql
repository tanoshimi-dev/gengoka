-- Seed initial admin user
-- Password: admin123 (CHANGE THIS IMMEDIATELY IN PRODUCTION!)
-- This hash is generated using argon2

INSERT INTO admin_users (email, password_hash, name, role, status)
VALUES (
    'admin@gengoka.com',
    -- This is the argon2 hash for 'admin123' - CHANGE THIS!
    '$argon2id$v=19$m=19456,t=2,p=1$xxxxxxxxxxxxxxxxxx$xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx',
    'Admin',
    'super_admin',
    'active'
) ON CONFLICT (email) DO NOTHING;

-- NOTE: The password hash above is a placeholder.
-- To generate a real password hash, you can use the Rust code:
--
-- use argon2::{
--     password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
--     Argon2,
-- };
--
-- fn main() {
--     let password = b"your_secure_password";
--     let salt = SaltString::generate(&mut OsRng);
--     let argon2 = Argon2::default();
--     let hash = argon2.hash_password(password, &salt).unwrap();
--     println!("{}", hash.to_string());
-- }
--
-- Or use the admin application itself to create users after the first one is set up.
