#[cfg(test)]
mod tests {
    use crate::frequency_block;
    use crate::logger;
    use crate::utils;

    const LOGLEVEL: &str = "Debug";
    const BIT_STRING_NIST_1: &str = "0110011010";
    const P_VALUE_NIST_1: f64 = 0.8012519569012013;
    const BIT_STRING_NIST_2: &str = "1100100100001111110110101010001000100001011010001100001000110100110001001100011001100010100010111000";
    const P_VALUE_NIST_2: f64 = 0.7064384496412821;
    const BIT_STRING_ONLY_ZEROS: &str = "0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
    const BIT_STRING_ONLY_ONES: &str = "1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111";
    const BIT_STRING_RANDOM: &str = "11101000100101110100010110100101111100000101010101000101110101010101011101101010010101000001011101110101";
    const BIT_STRING_NON_RANDOM: &str = "0000000000100000000000000000000000000100000000000000000000000000000010000000000000000000000000001000";
    const BIT_STRING_PERFECT: &str = "1111100000111110000011111000001111100000111110000011111000001111100000111110000011111000001111100000";
    const INVALID_BIT_STRING: &str = "010101111010101010101010101010a0101010101010100101010101";
    const PI_FILE: &str = "/src/tests/testdata/data.pi";
    const E_FILE: &str = "/src/tests/testdata/data.e";
    const SQRT_2_FILE: &str = "/src/tests/testdata/data.sqrt2";
    const SQRT_3_FILE: &str = "/src/tests/testdata/data.sqrt3";
    const SHA_3_FILE: &str = "/src/tests/testdata/data.sha3";

    #[test]
    fn test_frequency_block() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        assert_eq!(
            frequency_block::perform_test(BIT_STRING_NIST_1, 3).unwrap(),
            P_VALUE_NIST_1
        );
        assert_eq!(
            frequency_block::perform_test(BIT_STRING_NIST_2, 10).unwrap(),
            P_VALUE_NIST_2
        );
        assert!(frequency_block::perform_test(BIT_STRING_ONLY_ZEROS, 10).unwrap() < 0.01);
        assert!(frequency_block::perform_test(BIT_STRING_ONLY_ONES, 10).unwrap() < 0.01);
        assert_eq!(
            frequency_block::perform_test(BIT_STRING_ONLY_ZEROS, 10).unwrap(),
            frequency_block::perform_test(BIT_STRING_ONLY_ONES, 10).unwrap()
        );
        assert!(frequency_block::perform_test(BIT_STRING_RANDOM, 10).unwrap() >= 0.01);
        assert!(frequency_block::perform_test(BIT_STRING_NON_RANDOM, 20).unwrap() < 0.01);
        assert!(frequency_block::perform_test(BIT_STRING_PERFECT, 10).unwrap() == 1.00);

        // test pi, e, sqrt(2) and sqrt(3) in their respective binary representations
        let pi_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + PI_FILE;
        let pi_bit_string = utils::read_random_numbers(&pi_file).unwrap();
        assert!(frequency_block::perform_test(&pi_bit_string, 10200).unwrap() >= 0.01);

        let e_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + E_FILE;
        let e_bit_string = utils::read_random_numbers(&e_file).unwrap();
        assert!(frequency_block::perform_test(&e_bit_string, 10200).unwrap() >= 0.01);

        let sqrt_2_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + SQRT_2_FILE;
        let sqrt_2_bit_string = utils::read_random_numbers(&sqrt_2_file).unwrap();
        assert!(frequency_block::perform_test(&sqrt_2_bit_string, 10200).unwrap() >= 0.01);

        let sqrt_3_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + SQRT_3_FILE;
        let sqrt_3_bit_string = utils::read_random_numbers(&sqrt_3_file).unwrap();
        assert!(frequency_block::perform_test(&sqrt_3_bit_string, 10200).unwrap() >= 0.01);

        let sha_3_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + SHA_3_FILE;
        let sha_3_bit_string = utils::read_random_numbers(&sha_3_file).unwrap();
        assert!(frequency_block::perform_test(&sha_3_bit_string, 10250).unwrap() >= 0.01);
    }

    #[test]
    fn test_frequency_block_error_cases() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        let mut success: bool;

        // pass empty string
        match frequency_block::perform_test("", 10) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);

        // pass invalid bit string
        match frequency_block::perform_test(INVALID_BIT_STRING, 10) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);

        // pass wrong sizes of M
        match frequency_block::perform_test(BIT_STRING_NON_RANDOM, BIT_STRING_NON_RANDOM.len()) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);

        match frequency_block::perform_test(BIT_STRING_NON_RANDOM, 0) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);
    }
}
