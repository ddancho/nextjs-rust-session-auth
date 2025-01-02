use sha2::Digest;

pub fn hash_password(password: &str, password_salt: &str) -> String {
    let mut hasher = sha2::Sha512::new();

    hasher.update(password_salt);
    hasher.update(password);

    hex::encode(hasher.finalize())
}
