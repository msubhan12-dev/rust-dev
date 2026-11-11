use bcrypt::{hash, verify, DEFAULT_COST};

/// Hash password using bcrypt
pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

/// Verify password against hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let password = "MySecurePassword123!";
        let hash = hash_password(password).expect("Failed to hash password");

        // Hash should not equal password
        assert_ne!(hash, password);

        // Verify should succeed
        let is_valid = verify_password(password, &hash).expect("Failed to verify password");
        assert!(is_valid);
    }

    #[test]
    fn test_invalid_password() {
        let password = "MySecurePassword123!";
        let hash = hash_password(password).expect("Failed to hash password");
        let wrong_password = "WrongPassword";

        let is_valid = verify_password(wrong_password, &hash).expect("Failed to verify password");
        assert!(!is_valid);
    }
}
