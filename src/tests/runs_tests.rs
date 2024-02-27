#[cfg(test)]
mod tests {
    use crate::logger;
    use crate::runs;
    use serial_test::serial;

    const LOGLEVEL: &str = "Trace";
    const BIT_STRING_1: &str = "1001101011"; // Example from NIST paper. p-value should be 0.147232
    const BIT_STRING_2: &str = "0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
    const BIT_STRING_3: &str = "1100100100001111110110101010001000100001011010001100001000110100110001001100011001100010100010111000";
    const BIT_STRING_4: &str = "1010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010";
    const BIT_STRING_5: &str = "1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111";
    const BIT_STRING_6: &str = "1100110011001100110011001100110011001100110011001100110011001100110011001100110011001100110011001100";
    const INVALID_BIT_STRING: &str = "010101111010101010101010101010a0101010101010100101010101";

    #[test]
    #[serial]
    fn test_runs() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        assert!(runs::perform_test(BIT_STRING_1).unwrap() > 0.01);
        assert!(runs::perform_test(BIT_STRING_2).unwrap() <= 0.01);
        assert!(runs::perform_test(BIT_STRING_3).unwrap() > 0.01);
        assert!(runs::perform_test(BIT_STRING_4).unwrap() <= 0.01);
        assert!(runs::perform_test(BIT_STRING_5).unwrap() <= 0.01);
        assert!(runs::perform_test(BIT_STRING_6).unwrap() == 1.00);
    }

    #[test]
    #[serial]
    fn test_runs_error_cases() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        let mut success: bool;

        // pass empty string
        match runs::perform_test("") {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);

        // pass invalid bit string
        match runs::perform_test(INVALID_BIT_STRING) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);
    }
}
