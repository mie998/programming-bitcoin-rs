#[cfg(test)]
mod tests {
    use bs58::encode;
    use num_bigint::BigInt;

    #[test]
    fn bs58_test_1() {
        let byte_x = b"7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d";
        let v = BigInt::parse_bytes(byte_x, 16).unwrap();
        let (_, s) = v.to_bytes_be();
        assert_eq!(
            encode(s).into_string(),
            "9MA8fRQrT4u8Zj8ZRd6MAiiyaxb2Y1CMpvVkHQu5hVM6"
        );
    }

    #[test]
    fn bs58_test_2() {
        let byte_y = b"eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c";
        let v = BigInt::parse_bytes(byte_y, 16).unwrap();
        let (_, s) = v.to_bytes_be();
        assert_eq!(
            encode(s).into_string(),
            "4fE3H2E6XMp4SsxtwinF7w9a34ooUrwWe4WsW1458Pd"
        );
    }

    #[test]
    fn bs58_test_3() {
        let byte_z = b"c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6";
        let v = BigInt::parse_bytes(byte_z, 16).unwrap();
        let (_, s) = v.to_bytes_be();
        assert_eq!(
            encode(s).into_string(),
            "EQJsjkd6JaGwxrjEhfeqPenqHwrBmPQZjJGNSCHBkcF7"
        );
    }
}
