#[cfg(test)]
mod tests {
    use crate::cumulative_sums;
    use crate::customtypes;
    use crate::logger;
    use crate::utils;

    const LOGLEVEL: &str = "Debug";
    const BIT_STRING_NIST_1: &str = "1011010111";
    const P_VALUE_NIST_1: f64 = 0.4116586191729081;
    const BIT_STRING_NIST_2: &str = "1100100100001111110110101010001000100001011010001100001000110100110001001100011001100010100010111000";
    const P_VALUE_NIST_2_FORWARD: f64 = 0.2191939934949785;
    const P_VALUE_NIST_2_BACKWARD: f64 = 0.11486621529731965;
    const BIT_STRING_ONLY_ONES: &str = "1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111";
    const BIT_STRING_ONLY_ZEROS: &str = "0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
    const INVALID_BIT_STRING: &str = "1100110000010101011011000100110011100000000000100100110101010001000100a111010110100000001101011111001100111001101101100010110010";
    const PI_FILE: &str = "/src/tests/testdata/data.pi";
    const E_FILE: &str = "/src/tests/testdata/data.e";
    const SQRT_2_FILE: &str = "/src/tests/testdata/data.sqrt2";
    const SQRT_3_FILE: &str = "/src/tests/testdata/data.sqrt3";

    #[test]
    fn test_cumulative_sums() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        assert_eq!(
            cumulative_sums::perform_test(BIT_STRING_NIST_1, customtypes::Mode::Forward).unwrap(),
            P_VALUE_NIST_1
        );
        assert_eq!(
            cumulative_sums::perform_test(BIT_STRING_NIST_1, customtypes::Mode::Backward).unwrap(),
            P_VALUE_NIST_1
        );
        assert_eq!(
            cumulative_sums::perform_test(BIT_STRING_NIST_2, customtypes::Mode::Forward).unwrap(),
            P_VALUE_NIST_2_FORWARD
        );
        assert_eq!(
            cumulative_sums::perform_test(BIT_STRING_NIST_2, customtypes::Mode::Backward).unwrap(),
            P_VALUE_NIST_2_BACKWARD
        );
        assert!(
            cumulative_sums::perform_test(BIT_STRING_ONLY_ONES, customtypes::Mode::Forward)
                .unwrap()
                <= 0.01
        );
        assert!(
            cumulative_sums::perform_test(BIT_STRING_ONLY_ONES, customtypes::Mode::Backward)
                .unwrap()
                <= 0.01
        );
        assert!(
            cumulative_sums::perform_test(BIT_STRING_ONLY_ZEROS, customtypes::Mode::Forward)
                .unwrap()
                <= 0.01
        );
        assert!(
            cumulative_sums::perform_test(BIT_STRING_ONLY_ZEROS, customtypes::Mode::Backward)
                .unwrap()
                <= 0.01
        );
        assert_eq!(
            cumulative_sums::perform_test(BIT_STRING_ONLY_ZEROS, customtypes::Mode::Forward)
                .unwrap(),
            cumulative_sums::perform_test(BIT_STRING_ONLY_ONES, customtypes::Mode::Backward)
                .unwrap()
        );

        // test pi, e, sqrt(2) and sqrt(3) in their respective binary representations
        let pi_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + PI_FILE;
        let pi_bit_string = utils::read_random_numbers(&pi_file).unwrap();
        assert!(
            cumulative_sums::perform_test(&pi_bit_string, customtypes::Mode::Forward).unwrap()
                >= 0.01
        );
        assert!(
            cumulative_sums::perform_test(&pi_bit_string, customtypes::Mode::Backward).unwrap()
                >= 0.01
        );

        let e_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + E_FILE;
        let e_bit_string = utils::read_random_numbers(&e_file).unwrap();
        assert!(
            cumulative_sums::perform_test(&e_bit_string, customtypes::Mode::Forward).unwrap()
                >= 0.01
        );
        assert!(
            cumulative_sums::perform_test(&e_bit_string, customtypes::Mode::Backward).unwrap()
                >= 0.01
        );

        let sqrt_2_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + SQRT_2_FILE;
        let sqrt_2_bit_string = utils::read_random_numbers(&sqrt_2_file).unwrap();
        assert!(
            cumulative_sums::perform_test(&sqrt_2_bit_string, customtypes::Mode::Forward).unwrap()
                >= 0.01
        );
        assert!(
            cumulative_sums::perform_test(&sqrt_2_bit_string, customtypes::Mode::Backward).unwrap()
                >= 0.01
        );

        let sqrt_3_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + SQRT_3_FILE;
        let sqrt_3_bit_string = utils::read_random_numbers(&sqrt_3_file).unwrap();
        assert!(
            cumulative_sums::perform_test(&sqrt_3_bit_string, customtypes::Mode::Forward).unwrap()
                >= 0.01
        );
        assert!(
            cumulative_sums::perform_test(&sqrt_3_bit_string, customtypes::Mode::Backward).unwrap()
                >= 0.01
        );
    }

    #[test]
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
