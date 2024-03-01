#[cfg(test)]
mod tests {
    use crate::logger;
    use crate::longest_run;
    use crate::utils;
    use serial_test::serial;

    const LOGLEVEL: &str = "Debug";
    // Example from NIST paper. p-value should be 0.180598
    const BIT_STRING_1: &str = "11001100000101010110110001001100111000000000001001001101010100010001001111010110100000001101011111001100111001101101100010110010";
    const BIT_STRING_2: &str = "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
    const BIT_STRING_3: &str = "11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111";
    const BIT_STRING_4: &str = "10000000100000001000000010000000110000001100000011000000110000001110000011100000111000001110000011110000111100001111000011110000";
    const INVALID_BIT_STRING: &str = "1100110000010101011011000100110011100000000000100100110101010001000100a111010110100000001101011111001100111001101101100010110010";
    const NUMBER_OF_BYTES: usize = 125000;

    #[test]
    #[serial]
    fn test_longest_run() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        assert!(longest_run::perform_test(BIT_STRING_1).unwrap() >= 0.01);
        assert!(longest_run::perform_test(BIT_STRING_2).unwrap() < 0.01);
        assert!(longest_run::perform_test(BIT_STRING_3).unwrap() < 0.01);
        assert!(longest_run::perform_test(BIT_STRING_4).unwrap() >= 0.01);

        // test 1,000,000 newly generated random bits
        let random_bytes = utils::get_random_bytes(NUMBER_OF_BYTES).unwrap();
        let bit_string = utils::hex_bytes_to_bit_string(random_bytes).unwrap();
        assert!(longest_run::perform_test(&bit_string).unwrap() >= 0.01);
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
