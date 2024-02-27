#[cfg(test)]
mod tests {
    use crate::binary_matrix_rank;
    use crate::logger;
    use serial_test::serial;

    const LOGLEVEL: &str = "Trace";
    const BIT_STRING_1: &str = "01011001001010101101"; // example from NIST Paper. p-value should be 0.741948
    const INVALID_BIT_STRING: &str = "010101111010101010101010101010a0101010101010100101010101";

    #[test]
    #[serial]
    fn test_binary_matrix_rank() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        assert!(binary_matrix_rank::perform_test(BIT_STRING_1, 3, 3).unwrap() >= 0.01);
    }

    #[test]
    #[serial]
    fn test_binary_matrix_rank_error_cases() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        let mut success: bool;

        // pass empty string
        match binary_matrix_rank::perform_test("", 3, 3) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);

        // pass invalid bit string
        match binary_matrix_rank::perform_test(INVALID_BIT_STRING, 32, 32) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);
    }
}
