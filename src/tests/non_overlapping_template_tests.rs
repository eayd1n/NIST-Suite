#[cfg(test)]
mod tests {
    use crate::constants;
    use crate::logger;
    use crate::non_overlapping_template;
    use crate::utils;
    use serial_test::serial;

    const LOGLEVEL: &str = "Trace";
    const BIT_STRING_1: &str = "10100100101110010110"; // example from NIST Paper. p-value should be 0.344154
    const INVALID_BIT_STRING: &str = "010101111010101010101010101010a0101010101010100101010101";
    const PI_FILE: &str = "/src/tests/testdata/data.pi";
    const E_FILE: &str = "/src/tests/testdata/data.e";
    const SQRT_2_FILE: &str = "/src/tests/testdata/data.sqrt2";
    const SQRT_3_FILE: &str = "/src/tests/testdata/data.sqrt3";

    #[test]
    #[serial]
    fn test_non_overlapping_template() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        assert!(non_overlapping_template::perform_test(BIT_STRING_1, 3, 2).unwrap() >= 0.01);

        // test pi, e, sqrt(2) and sqrt(3) in their respective binary representations
        let pi_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + PI_FILE;
        let pi_bit_string = utils::read_random_numbers(&pi_file).unwrap();
        assert!(
            non_overlapping_template::perform_test(
                &pi_bit_string,
                10,
                constants::N_BLOCKS_NON_OVERLAPPING_TEMPLATE
            )
            .unwrap()
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
            non_overlapping_template::perform_test(
                &e_bit_string,
                10,
                constants::N_BLOCKS_NON_OVERLAPPING_TEMPLATE
            )
            .unwrap()
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
            non_overlapping_template::perform_test(
                &sqrt_2_bit_string,
                10,
                constants::N_BLOCKS_NON_OVERLAPPING_TEMPLATE
            )
            .unwrap()
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
            non_overlapping_template::perform_test(
                &sqrt_3_bit_string,
                10,
                constants::N_BLOCKS_NON_OVERLAPPING_TEMPLATE
            )
            .unwrap()
                >= 0.01
        );
    }

    #[test]
    #[serial]
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

        // pass invalid template length sizes
        match non_overlapping_template::perform_test(BIT_STRING_1, 0, 4) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);

        match non_overlapping_template::perform_test(INVALID_BIT_STRING, 11, 3) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);
    }
}
