#[cfg(test)]
mod tests {
    use crate::binary_matrix_rank;
    use crate::constants;
    use crate::logger;
    use crate::utils;

    const LOGLEVEL: &str = "Debug";
    const BIT_STRING_1: &str = "01011001001010101101"; // example from NIST Paper. p-value should be 0.741948
    const INVALID_BIT_STRING: &str = "010101111010101010101010101010a0101010101010100101010101";
    const PI_FILE: &str = "/src/tests/testdata/data.pi";
    const E_FILE: &str = "/src/tests/testdata/data.e";
    const SQRT_2_FILE: &str = "/src/tests/testdata/data.sqrt2";
    const SQRT_3_FILE: &str = "/src/tests/testdata/data.sqrt3";
    const SHA_3_FILE: &str = "/src/tests/testdata/data.sha3";

    // XXX Fix binary matrix rank
    #[test]
    fn test_binary_matrix_rank() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        assert!(binary_matrix_rank::perform_test(BIT_STRING_1, 3, 3).unwrap() >= 0.01);

        // test pi, e, sqrt(2) and sqrt(3) in their respective binary representations
        let pi_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + PI_FILE;
        let pi_bit_string = utils::read_random_numbers(&pi_file).unwrap();
        assert!(
            binary_matrix_rank::perform_test(
                &pi_bit_string,
                constants::MATRIX_ROWS_M,
                constants::MATRIX_COLUMNS_Q
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
            binary_matrix_rank::perform_test(
                &e_bit_string,
                constants::MATRIX_ROWS_M,
                constants::MATRIX_COLUMNS_Q
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
            binary_matrix_rank::perform_test(
                &sqrt_2_bit_string,
                constants::MATRIX_ROWS_M,
                constants::MATRIX_COLUMNS_Q
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
            binary_matrix_rank::perform_test(
                &sqrt_3_bit_string,
                constants::MATRIX_ROWS_M,
                constants::MATRIX_COLUMNS_Q
            )
            .unwrap()
                >= 0.01
        );

        let sha_3_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + SHA_3_FILE;
        let sha_3_bit_string = utils::read_random_numbers(&sha_3_file).unwrap();
        assert!(
            binary_matrix_rank::perform_test(
                &sha_3_bit_string,
                constants::MATRIX_ROWS_M,
                constants::MATRIX_COLUMNS_Q
            )
            .unwrap()
                >= 0.01
        );
    }

    #[test]
    fn test_binary_matrix_rank_error_cases() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        let mut success: bool;

        // pass empty string
        match binary_matrix_rank::perform_test("", 3, 3) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);

        // pass invalid bit string
        match binary_matrix_rank::perform_test(INVALID_BIT_STRING, 32, 32) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);
    }
}
