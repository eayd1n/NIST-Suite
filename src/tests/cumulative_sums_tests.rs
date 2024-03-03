#[cfg(test)]
mod tests {
    use crate::cumulative_sums;
    use crate::customtypes;
    use crate::logger;
    use crate::utils;
    use serial_test::serial;

    const LOGLEVEL: &str = "Debug";
    const BIT_STRING_1: &str = "1011010111"; // Example from NIST Paper. p-value should be 0.4116586
    const BIT_STRING_2: &str = "1100100100001111110110101010001000100001011010001100001000110100110001001100011001100010100010111000";
    const INVALID_BIT_STRING: &str = "1100110000010101011011000100110011100000000000100100110101010001000100a111010110100000001101011111001100111001101101100010110010";
    const NUMBER_OF_BYTES: usize = 1250;

    #[test]
    #[serial]
    fn test_cumulative_sums() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        assert!(
            cumulative_sums::perform_test(BIT_STRING_1, customtypes::Mode::Forward).unwrap() > 0.01
        );
        assert!(
            cumulative_sums::perform_test(BIT_STRING_1, customtypes::Mode::Backward).unwrap()
                > 0.01
        );
        assert!(
            cumulative_sums::perform_test(BIT_STRING_2, customtypes::Mode::Forward).unwrap() > 0.01
        );
        assert!(
            cumulative_sums::perform_test(BIT_STRING_2, customtypes::Mode::Backward).unwrap()
                > 0.01
        );

        // test 10,000 newly generated random bits (too long size takes a long time here)
        let random_bytes = utils::get_random_bytes(NUMBER_OF_BYTES).unwrap();
        let bit_string = utils::hex_bytes_to_bit_string(random_bytes).unwrap();
        assert!(
            cumulative_sums::perform_test(&bit_string, customtypes::Mode::Forward).unwrap() >= 0.01
        );
        assert!(
            cumulative_sums::perform_test(&bit_string, customtypes::Mode::Backward).unwrap()
                >= 0.01
        );
    }

    #[test]
    #[serial]
    fn test_cumulative_sums_error_cases() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        let mut success: bool;

        // pass empty string
        match cumulative_sums::perform_test("", customtypes::Mode::Backward) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);

        // pass invalid bit string
        match cumulative_sums::perform_test(INVALID_BIT_STRING, customtypes::Mode::Forward) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);
    }
}
