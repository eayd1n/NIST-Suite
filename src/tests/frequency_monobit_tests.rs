#[cfg(test)]
mod tests {
    use crate::frequency_monobit;
    use crate::logger;
    use serial_test::serial;

    const LOGLEVEL: &str = "Trace";
    const BIT_STRING_1: &str = "1011010101";
    const BIT_STRING_2: &str = "0000000000000000000000000000000000000000000000000000000000000";
    const BIT_STRING_3: &str = "10101010101111111111111";
    const BIT_STRING_4: &str = "1100100100001111110110101010001000100001011010001100001000110100110001001100011001100010100010111000";
    const INVALID_BIT_STRING: &str = "010101111010101010101010101010a0101010101010100101010101";

    #[test]
    #[serial]
    fn test_hex_to_bit_string() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        assert!(frequency_monobit::perform_test(BIT_STRING_1).unwrap());
        assert!(!frequency_monobit::perform_test(BIT_STRING_2).unwrap());
        assert!(!frequency_monobit::perform_test(BIT_STRING_3).unwrap());
        assert!(frequency_monobit::perform_test(BIT_STRING_4).unwrap());
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
