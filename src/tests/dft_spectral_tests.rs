#[cfg(test)]
mod tests {
    use crate::dft_spectral;
    use crate::logger;
    use crate::utils;
    use serial_test::serial;

    const LOGLEVEL: &str = "Debug";
    const BIT_STRING_1: &str = "1001010011"; // example from NIST Paper. p-value should be 0.029523
    const BIT_STRING_2: &str = "1100100100001111110110101010001000100001011010001100001000110100110001001100011001100010100010111000";
    const INVALID_BIT_STRING: &str = "010101111010101010101010101010a0101010101010100101010101";
    const NUMBER_OF_BYTES: usize = 125000;

    #[test]
    #[serial]
    fn test_dft_spectral() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        //  XXX Fix dicrete fourier transform
        assert!(dft_spectral::perform_test(BIT_STRING_1).unwrap() != 1.00);
        assert!(dft_spectral::perform_test(BIT_STRING_2).unwrap() != 1.00);

        // test 1,000,000 newly generated random bits
        let random_bytes = utils::get_random_bytes(NUMBER_OF_BYTES).unwrap();
        let bit_string = utils::hex_bytes_to_bit_string(random_bytes).unwrap();
        assert!(dft_spectral::perform_test(&bit_string).unwrap() != 1.00);
    }

    #[test]
    #[serial]
    fn test_dft_spectral_error_cases() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        let mut success: bool;

        // pass empty string
        match dft_spectral::perform_test("") {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);

        // pass invalid bit string
        match dft_spectral::perform_test(INVALID_BIT_STRING) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);
    }
}
