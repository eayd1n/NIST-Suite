#[cfg(test)]
mod tests {
    use crate::frequency_block;
    use crate::logger;
    use serial_test::serial;

    const LOGLEVEL: &str = "Trace";
    const BIT_STRING_1: &str = "0110011010";
    const BIT_STRING_2: &str = "0000000000000000000000000000000000000000000000000000000000000";
    const BIT_STRING_3: &str = "1100100100001111110110101010001000100001011010001100001000110100110001001100011001100010100010111000";
    const INVALID_BIT_STRING: &str = "010101111010101010101010101010a0101010101010100101010101";

    #[test]
    #[serial]
    fn test_frequency_block() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        assert!(frequency_block::perform_test(BIT_STRING_1).unwrap());
        assert!(!frequency_block::perform_test(BIT_STRING_2).unwrap());
        assert!(frequency_block::perform_test(BIT_STRING_3).unwrap());
    }

    #[test]
    #[serial]
    fn test_frequency_block_error_cases() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        let mut success: bool;

        // pass empty string
        match frequency_block::perform_test("") {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);

        // pass invalid bit string
        match frequency_block::perform_test(INVALID_BIT_STRING) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);
    }
}
