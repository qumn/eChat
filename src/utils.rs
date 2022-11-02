use std::num::NonZeroU32;

use data_encoding::BASE64;
use ring::{
    digest,
    error::Unspecified,
    pbkdf2,
    rand::{self, SecureRandom},
};

const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
// given a password, encyption the password with a salt
// that be generated randomly. and return the encyption password and the salt
pub fn encyption(password: &str) -> Result<(String, String), Unspecified> {
    let mut salt = [0u8; CREDENTIAL_LEN];
    let n_iter = NonZeroU32::new(100_000).unwrap(); // 密码迭代加密的次数
    let rng = rand::SystemRandom::new();
    rng.fill(&mut salt)?;
    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &mut pbkdf2_hash,
    );
    Ok((BASE64.encode(&pbkdf2_hash), BASE64.encode(&salt)))
}
pub fn verify(password: &str, encyption_password: &str, salt: &str) -> bool {
    let n_iter = NonZeroU32::new(100_000).unwrap();
    let salt = BASE64.decode(salt.as_bytes()).unwrap();
    let encyption_password = BASE64.decode(encyption_password.as_bytes()).unwrap();
    pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt[..],
        password.as_bytes(),
        &encyption_password[..],
    )
    .is_ok()
}
#[cfg(test)]
mod test {
    use super::{encyption, verify};
    #[test]
    fn encyption_should_work() {
        let password = "123";
        let (encyption_password, salt) = encyption(password).unwrap();
        assert!(verify(&password, &encyption_password, &salt));
    }
}