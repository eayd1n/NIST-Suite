#[cfg(test)]
mod tests {
    use crate::cumulative_sums;
    use crate::logger;
    use serial_test::serial;

    const LOGLEVEL: &str = "Trace";
    const BIT_STRING_1: &str = "1011010111";
    const BIT_STRING_2: &str = "1100100100001111110110101010001000100001011010001100001000110100110001001100011001100010100010111000";
    const INVALID_BIT_STRING: &str = "1100110000010101011011000100110011100000000000100100110101010001000100a111010110100000001101011111001100111001101101100010110010";

    #[test]
    #[serial]
    fn test_cumulative_sums() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");
        /*
                assert!(
                    cumulative_sums::perform_test(BIT_STRING_1, cumulative_sums::Mode::Forward).unwrap()
                        > 0.01
                );
                assert!(
                    cumulative_sums::perform_test(BIT_STRING_1, cumulative_sums::Mode::Backward).unwrap()
                        > 0.01
                );
                assert!(
                    cumulative_sums::perform_test(BIT_STRING_2, cumulative_sums::Mode::Forward).unwrap()
                        > 0.01
                );
                assert!(
                    cumulative_sums::perform_test(BIT_STRING_2, cumulative_sums::Mode::Backward).unwrap()
                        > 0.01
                );
        */
    }

    #[test]
    #[serial]
    fn test_cumulative_sums_error_cases() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        let mut success: bool;

        // pass empty string
        match cumulative_sums::perform_test("", cumulative_sums::Mode::Backward) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);

        // pass invalid bit string
        match cumulative_sums::perform_test(INVALID_BIT_STRING, cumulative_sums::Mode::Forward) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);
    }
}
