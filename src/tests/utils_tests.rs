#[cfg(test)]
mod tests {
    use crate::logger;
    use crate::utils;
    use serial_test::serial;

    const LOGLEVEL: &str = "Trace";
    const HEX_STRING_1: &str = "0xab00ded6f3c0";
    const HEX_STRING_2: &str = "00FFDE89C03DA6C2B5";
    const BIT_STRING_1: &str = "101010110000000011011110110101101111001111000000";
    const BIT_STRING_2: &str =
        "000000001111111111011110100010011100000000111101101001101100001010110101";
    const INVALID_HEX_STRING: &str = "0xelrgjherlkgjerlkgjerlgjerlkgjerlgj";

    #[test]
    #[serial]
    fn test_hex_to_bit_string() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        assert_eq!(
            utils::hex_to_bit_string(HEX_STRING_1).unwrap(),
            BIT_STRING_1
        );
        assert_eq!(
            utils::hex_to_bit_string(HEX_STRING_2).unwrap(),
            BIT_STRING_2
        );
    }

    #[test]
    #[serial]
    fn test_utils_error_cases() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        let mut success: bool;

        // pass empty string
        match utils::hex_to_bit_string("") {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);

        // pass invalid hex string
        match utils::hex_to_bit_string(INVALID_HEX_STRING) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);
    }
}
