#[cfg(test)]
mod tests {
    use crate::frequency_monobit;
    use crate::logger;
    use crate::utils;
    use serial_test::serial;

    const LOGLEVEL: &str = "Debug";
    const BIT_STRING_1: &str = "1011010101"; // example from NIST Paper. p-value should be 0.527089
    const BIT_STRING_2: &str = "0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
    const BIT_STRING_3: &str = "10101010101111111111111";
    const BIT_STRING_4: &str = "1100100100001111110110101010001000100001011010001100001000110100110001001100011001100010100010111000";
    const BIT_STRING_5: &str = "1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111";
    const BIT_STRING_6: &str = "11001001000011111101101010100010001000010110100011000010001101001100010011000110011000101000101110001100100100001111110110101010001000100001011010001100001000110100110001001100011001100010100010111000";
    const BIT_STRING_7: &str = "0000000000000000000000000000000000000000000000000011111111111111111111111111111111111111111111111111";
    const INVALID_BIT_STRING: &str = "010101111010101010101010101010a0101010101010100101010101";
    const NUMBER_OF_BYTES: usize = 125000;

    #[test]
    #[serial]
    fn test_frequency_monobit() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        assert!(frequency_monobit::perform_test(BIT_STRING_1).unwrap() >= 0.01);
        assert!(frequency_monobit::perform_test(BIT_STRING_2).unwrap() < 0.01);
        assert!(frequency_monobit::perform_test(BIT_STRING_3).unwrap() < 0.01);
        assert!(frequency_monobit::perform_test(BIT_STRING_4).unwrap() >= 0.01);
        assert!(frequency_monobit::perform_test(BIT_STRING_5).unwrap() < 0.01);
        assert!(frequency_monobit::perform_test(BIT_STRING_6).unwrap() >= 0.01);
        assert!(frequency_monobit::perform_test(BIT_STRING_7).unwrap() == 1.00);

        // test 1,000,000 newly generated random bits
        let random_bytes = utils::get_random_bytes(NUMBER_OF_BYTES).unwrap();
        let bit_string = utils::hex_bytes_to_bit_string(random_bytes).unwrap();
        assert!(frequency_monobit::perform_test(&bit_string).unwrap() >= 0.01);
    }

    #[test]
    #[serial]
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
