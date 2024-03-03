#[cfg(test)]
mod tests {
    use crate::binary_matrix_rank;
    use crate::constants;
    use crate::logger;
    use crate::utils;
    use rug::{ops::Pow, Float, Integer};
    use serial_test::serial;

    const LOGLEVEL: &str = "Debug";
    const BIT_STRING_1: &str = "01011001001010101101"; // example from NIST Paper. p-value should be 0.741948
    const INVALID_BIT_STRING: &str = "010101111010101010101010101010a0101010101010100101010101";
    const NUMBER_OF_BYTES: usize = 12500;

    // XXX Fix binary matrix rank
    #[test]
    #[serial]
    fn test_binary_matrix_rank() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        assert!(binary_matrix_rank::perform_test(BIT_STRING_1, 3, 3).unwrap() >= 0.01);

        // Set precision to 100,000 binary digits
        let mut e = Float::with_val(100_000, 0.0);
        e.set_prec(100_000);

        // Calculate e with desired precision
        e = Float::with_val(100_000, 1).exp();

        // Extract the fractional part by subtracting the integer part
        let fractional_part = e.clone() - e.floor();

        // Multiply the fractional part by 2^100,000 to extract the binary digits
        let multiplied: Float = fractional_part * Float::with_val(1, 2).pow(100_000);

        // Convert the multiplied value to an integer
        let multiplied_integer: Integer = multiplied.to_integer().unwrap();

        // Convert the integer to its binary representation as a string
        let binary_digits = multiplied_integer.to_string_radix(2);

        assert!(
            binary_matrix_rank::perform_test(
                &binary_digits,
                constants::MATRIX_ROWS_M,
                constants::MATRIX_COLUMNS_Q
            )
            .unwrap()
                != 0.01
        );

        // test 100,000 newly generated random bits
        let random_bytes = utils::get_random_bytes(NUMBER_OF_BYTES).unwrap();
        let bit_string = utils::hex_bytes_to_bit_string(random_bytes).unwrap();
        assert!(
            binary_matrix_rank::perform_test(
                &bit_string,
                constants::MATRIX_ROWS_M,
                constants::MATRIX_COLUMNS_Q
            )
            .unwrap()
                != 0.01
        );
    }

    #[test]
    #[serial]
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
