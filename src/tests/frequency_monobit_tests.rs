#[cfg(test)]
mod tests {
    use crate::frequency_monobit;
    use crate::logger;
    use crate::utils;

    const LOGLEVEL: &str = "Debug";
    const BIT_STRING_NIST_1: &str = "1011010101";
    const P_VALUE_NIST_1: f64 = 0.5270892568655381;
    const BIT_STRING_NIST_2: &str = "1100100100001111110110101010001000100001011010001100001000110100110001001100011001100010100010111000";
    const P_VALUE_NIST_2: f64 = 0.10959858340379211;
    const BIT_STRING_ONLY_ZEROS: &str = "0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
    const BIT_STRING_ONLY_ONES: &str = "1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111";
    const BIT_STRING_NON_RANDOM: &str = "10101010101111111111111";
    const BIT_STRING_RANDOM: &str = "11001001000011111101101010100010001000010110100011000010001101001100010011000110011000101000101110001100100100001111110110101010001000100001011010001100001000110100110001001100011001100010100010111000";
    const BIT_STRING_PERFECT: &str = "0000000000000000000000000000000000000000000000000011111111111111111111111111111111111111111111111111";
    const INVALID_BIT_STRING: &str = "010101111010101010101010101010a0101010101010100101010101";
    const PI_FILE: &str = "/src/tests/testdata/data.pi";
    const E_FILE: &str = "/src/tests/testdata/data.e";
    const SQRT_2_FILE: &str = "/src/tests/testdata/data.sqrt2";
    const SQRT_3_FILE: &str = "/src/tests/testdata/data.sqrt3";
    const SHA_3_FILE: &str = "/src/tests/testdata/data.sha3";

    #[test]
    fn test_frequency_monobit() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        assert_eq!(
            frequency_monobit::perform_test(BIT_STRING_NIST_1).unwrap(),
            P_VALUE_NIST_1
        );
        assert_eq!(
            frequency_monobit::perform_test(BIT_STRING_NIST_2).unwrap(),
            P_VALUE_NIST_2
        );
        assert!(frequency_monobit::perform_test(BIT_STRING_ONLY_ZEROS).unwrap() < 0.01);
        assert!(frequency_monobit::perform_test(BIT_STRING_ONLY_ONES).unwrap() < 0.01);
        assert_eq!(
            frequency_monobit::perform_test(BIT_STRING_ONLY_ZEROS).unwrap(),
            frequency_monobit::perform_test(BIT_STRING_ONLY_ONES).unwrap()
        );
        assert!(frequency_monobit::perform_test(BIT_STRING_NON_RANDOM).unwrap() < 0.01);
        assert!(frequency_monobit::perform_test(BIT_STRING_RANDOM).unwrap() >= 0.01);
        assert!(frequency_monobit::perform_test(BIT_STRING_PERFECT).unwrap() == 1.00);

        // test pi, e, sqrt(2) and sqrt(3) in their respective binary representations
        let pi_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + PI_FILE;
        let pi_bit_string = utils::read_random_numbers(&pi_file).unwrap();
        assert!(frequency_monobit::perform_test(&pi_bit_string).unwrap() >= 0.01);

        let e_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + E_FILE;
        let e_bit_string = utils::read_random_numbers(&e_file).unwrap();
        assert!(frequency_monobit::perform_test(&e_bit_string).unwrap() >= 0.01);

        let sqrt_2_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + SQRT_2_FILE;
        let sqrt_2_bit_string = utils::read_random_numbers(&sqrt_2_file).unwrap();
        assert!(frequency_monobit::perform_test(&sqrt_2_bit_string).unwrap() >= 0.01);

        let sqrt_3_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + SQRT_3_FILE;
        let sqrt_3_bit_string = utils::read_random_numbers(&sqrt_3_file).unwrap();
        assert!(frequency_monobit::perform_test(&sqrt_3_bit_string).unwrap() >= 0.01);

        let sha_3_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + SHA_3_FILE;
        let sha_3_bit_string = utils::read_random_numbers(&sha_3_file).unwrap();
        assert!(frequency_monobit::perform_test(&sha_3_bit_string).unwrap() >= 0.01);
    }

    #[test]
    fn test_frequency_monobit_error_cases() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        let mut success: bool;

        // pass empty string
        match frequency_monobit::perform_test("") {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);

        // pass invalid bit string
        match frequency_monobit::perform_test(INVALID_BIT_STRING) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);
    }
}
