use argon2::{Argon2, PasswordHash, PasswordVerifier};
use password_hash::Error;

fn verify_password(password: &str, hash: &str) -> Result<bool, Error> {
    // mysql db 패스워드 해시
    let parsed_hash = PasswordHash::new(hash)?;
    let argon2 = Argon2::default();
    Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
}