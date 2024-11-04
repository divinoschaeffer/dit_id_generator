use sha2::{Sha256,Digest};


/// Hashes the provided content using SHA-256 and returns a hexadecimal-encoded string.
///
/// The `generate` function takes a `String` as input, hashes its contents using the SHA-256 algorithm,
/// and returns the resulting hash as a hexadecimal-encoded `String`. This function is useful for generating
/// unique, consistent identifiers based on the input content.
///
/// # Parameters
///
/// - `content`: A `String` containing the data to be hashed.
///
/// # Returns
///
/// A `String` representing the hexadecimal-encoded SHA-256 hash of the input content.
pub fn generate(content: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    let hash = hasher.finalize();
    hex::encode(hash)
}

#[cfg(test)]
mod tests {
    use crate::features::generator::generate;

    #[test]
    fn should_generate_id(){
        let result = generate(String::from("hello world"));
        assert_eq!(
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9",
            result
        )
    }
}