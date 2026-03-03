use crate::errors::AppError;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

// const ARGON2_VERSION: u32 = 0x13; // Версия 19 (0x13)
const ARGON2_M_COST: u32 = 8192; // Память в KiB (19 MB)
const ARGON2_T_COST: u32 = 1; // Время (итерации)
const ARGON2_P_COST: u32 = 1; // Параллелизм

type PasswordResult<T> = std::result::Result<T, AppError>;

fn argon2() -> Argon2<'static> {
    Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        argon2::Params::new(ARGON2_M_COST, ARGON2_T_COST, ARGON2_P_COST, None).unwrap(),
    )
}

pub fn hash_password(password: &str) -> PasswordResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = argon2();
    let hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(hash.to_string())
}

pub fn hash_refresh_token(token: &str) -> PasswordResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    // let salt = SaltString::from_b64("X3JmcmVzaF90b2tlbl9zYWx0")?;

    // Для refresh token используем меньшие параметры для скорости
    let argon2 = Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        argon2::Params::new(8192, 1, 1, None).unwrap(), // Меньше памяти и итераций
    );
    let hash = argon2.hash_password(token.as_bytes(), &salt)?;
    Ok(hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> PasswordResult<bool> {
    let parsed_hash = PasswordHash::new(hash)?;
    let argon2 = argon2();
    let verified = argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();
    Ok(verified)
}
