#[cfg(test)]
mod tests {
    use crate::utils::AppHasher;

    const HASH_LENGTH: usize = 20;

    #[tokio::test]
    async fn test_hasher() {
        let test_string = "test_string";
        let hashed_first = test_string.hash().await;
        let hashed_second = test_string.hash().await;

        assert_eq!(hashed_first.len(), HASH_LENGTH);
        assert_ne!(test_string, hashed_first);
        assert_eq!(hashed_first, hashed_second);
    }
}
