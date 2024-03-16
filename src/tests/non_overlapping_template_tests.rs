#[cfg(test)]
mod tests {
    use crate::constants;
    use crate::logger;
    use crate::non_overlapping_template;
    use crate::utils;

    const LOGLEVEL: &str = "Debug";
    const BIT_STRING_NIST_1: &str = "10100100101110010110";
    const BIT_STRING_ONLY_ZEROS: &str = "0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
    const BIT_STRING_ONLY_ONES: &str = "1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111";
    const BIT_STRING_SAME_PATTERN: &str = "1101101101101101101101101101101101101101101101101101101101101101101101101101101101101101101101101101";
    const INVALID_BIT_STRING: &str = "010101111010101010101010101010a0101010101010100101010101";
    const PI_FILE: &str = "/src/tests/testdata/data.pi";
    const E_FILE: &str = "/src/tests/testdata/data.e";
    const SQRT_2_FILE: &str = "/src/tests/testdata/data.sqrt2";
    const SQRT_3_FILE: &str = "/src/tests/testdata/data.sqrt3";
    const SHA_3_FILE: &str = "/src/tests/testdata/data.sha3";

    #[test]
    fn test_non_overlapping_template() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        assert!(non_overlapping_template::perform_test(BIT_STRING_NIST_1, 3, 2).unwrap() > 0.01);
        /*   assert!(
                    non_overlapping_template::perform_test(BIT_STRING_SAME_PATTERN, 3, 2).unwrap() <= 0.01
                );
        */
        // test pi, e, sqrt(2) and sqrt(3) in their respective binary representations
        let pi_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + PI_FILE;
        let pi_bit_string = utils::read_random_numbers(&pi_file).unwrap();
        assert!(non_overlapping_template::perform_test(&pi_bit_string, 10, 8).unwrap() >= 0.01);

        let e_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + E_FILE;
        let e_bit_string = utils::read_random_numbers(&e_file).unwrap();
        assert!(non_overlapping_template::perform_test(&e_bit_string, 10, 8).unwrap() >= 0.01);

        let sqrt_2_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + SQRT_2_FILE;
        let sqrt_2_bit_string = utils::read_random_numbers(&sqrt_2_file).unwrap();
        assert!(non_overlapping_template::perform_test(&sqrt_2_bit_string, 10, 8).unwrap() >= 0.01);

        let sqrt_3_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + SQRT_3_FILE;
        let sqrt_3_bit_string = utils::read_random_numbers(&sqrt_3_file).unwrap();
        assert!(non_overlapping_template::perform_test(&sqrt_3_bit_string, 10, 8).unwrap() >= 0.01);

        let sha_3_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + SHA_3_FILE;
        let sha_3_bit_string = utils::read_random_numbers(&sha_3_file).unwrap();
        assert!(non_overlapping_template::perform_test(&sha_3_bit_string, 10, 8).unwrap() >= 0.01);
    }

    #[test]
    fn test_non_overlapping_template_error_cases() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        let mut success: bool;

        // pass empty string
        match non_overlapping_template::perform_test("", 3, 2) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);

        // pass invalid bit string
        match non_overlapping_template::perform_test(INVALID_BIT_STRING, 6, 2) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);

        // pass only zeros or only ones
        match non_overlapping_template::perform_test(BIT_STRING_ONLY_ZEROS, 4, 2) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);

        match non_overlapping_template::perform_test(BIT_STRING_ONLY_ONES, 4, 2) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);

        // pass invalid template length sizes
        match non_overlapping_template::perform_test(BIT_STRING_NIST_1, 0, 4) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);

        match non_overlapping_template::perform_test(BIT_STRING_NIST_1, 22, 3) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);

        // pass invalid number of blocks size
        match non_overlapping_template::perform_test(BIT_STRING_NIST_1, 3, 120) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);
    }
}
