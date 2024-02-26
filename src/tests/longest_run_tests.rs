#[cfg(test)]
mod tests {
    use crate::logger;
    use crate::longest_run;
    use serial_test::serial;

    const LOGLEVEL: &str = "Trace";
    const BIT_STRING_1: &str = "11001100000101010110110001001100111000000000001001001101010100010001001111010110100000001101011111001100111001101101100010110010";
    const BIT_STRING_2: &str = "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
    const BIT_STRING_3: &str = "11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111";
    const INVALID_BIT_STRING: &str = "1100110000010101011011000100110011100000000000100100110101010001000100a111010110100000001101011111001100111001101101100010110010";

    #[test]
    #[serial]
    fn test_longest_run() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        assert!(longest_run::perform_test(BIT_STRING_1).unwrap() >= 0.01);
        // although both bit strings are obviously not random, longest run test can not detect it
        assert!(longest_run::perform_test(BIT_STRING_2).unwrap() >= 0.01);
        assert!(longest_run::perform_test(BIT_STRING_3).unwrap() >= 0.01);
    }

    #[test]
    #[serial]
    fn test_longest_run_error_cases() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        let mut success: bool;

        // pass empty string
        match longest_run::perform_test("") {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);

        // pass invalid bit string
        match longest_run::perform_test(INVALID_BIT_STRING) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);
    }
}
