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
    const PI_FILE: &str = "/src/tests/testdata/data.pi";
    const E_FILE: &str = "/src/tests/testdata/data.e";
    const SQRT_2_FILE: &str = "/src/tests/testdata/data.sqrt2";
    const SQRT_3_FILE: &str = "/src/tests/testdata/data.sqrt3";

    #[test]
    #[serial]
    fn test_longest_run() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        assert!(longest_run::perform_test(BIT_STRING_1).unwrap() >= 0.01);
        assert!(longest_run::perform_test(BIT_STRING_2).unwrap() < 0.01);
        assert!(longest_run::perform_test(BIT_STRING_3).unwrap() < 0.01);
        assert!(longest_run::perform_test(BIT_STRING_4).unwrap() >= 0.01);

        // test pi, e, sqrt(2) and sqrt(3) in their respective binary representations
        let pi_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + PI_FILE;
        let pi_bit_string = utils::read_random_numbers(&pi_file).unwrap();
        assert!(longest_run::perform_test(&pi_bit_string).unwrap() >= 0.01);

        let e_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + E_FILE;
        let e_bit_string = utils::read_random_numbers(&e_file).unwrap();
        assert!(longest_run::perform_test(&e_bit_string).unwrap() >= 0.01);

        let sqrt_2_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + SQRT_2_FILE;
        let sqrt_2_bit_string = utils::read_random_numbers(&sqrt_2_file).unwrap();
        assert!(longest_run::perform_test(&sqrt_2_bit_string).unwrap() >= 0.01);

        let sqrt_3_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + SQRT_3_FILE;
        let sqrt_3_bit_string = utils::read_random_numbers(&sqrt_3_file).unwrap();
        assert!(longest_run::perform_test(&sqrt_3_bit_string).unwrap() >= 0.01);
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
