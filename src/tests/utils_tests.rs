#[cfg(test)]
mod tests {
    use crate::customtypes;
    use crate::logger;
    use crate::utils;

    const LOGLEVEL: &str = "Trace";
    const TEST_NAME: customtypes::Test = customtypes::Test::FrequencyMonobit;
    static RANDOM_BYTES_1: [u8; 6] = [0xab, 0x00, 0xde, 0xd6, 0xf3, 0xc0];
    static RANDOM_BYTES_2: [u8; 9] = [0x00, 0xFF, 0xDE, 0x89, 0xC0, 0x3D, 0xA6, 0xC2, 0xB5];
    const BIT_STRING_1: &str = "101010110000000011011110110101101111001111000000";
    const BIT_STRING_2: &str =
        "000000001111111111011110100010011100000000111101101001101100001010110101";
    const INVALID_BIT_STRING: &str = "010101010101011110101010101010101X0101010101010101010101";
    const NUMBER_OF_BYTES: usize = 100;
    const HEX_BYTES_FILE: &str = "/src/tests/testdata/random_hex_bytes";
    const BIT_STRING_FROM_FILE: &str = "110010101111111010111010101111101101111010101101101111101110111111000000110111101010111111111110";
    const BIT_STRING_FILE: &str = "/src/tests/testdata/random_bit_string";
    const INVALID_CHAR_IN_FILE: &str = "/src/tests/testdata/random_invalid_char";
    const INVALID_FILE: &str = "/non-existing-dir/random_numbers";

    #[test]
    fn test_hex_bytes_to_bit_string() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        assert_eq!(
            utils::hex_bytes_to_bit_string(RANDOM_BYTES_1.to_vec()).unwrap(),
            BIT_STRING_1
        );
        assert_eq!(
            utils::hex_bytes_to_bit_string(RANDOM_BYTES_2.to_vec()).unwrap(),
            BIT_STRING_2
        );

        let success: bool;

        // pass empty vector
        match utils::hex_bytes_to_bit_string(Vec::<u8>::new()) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);
    }

    #[test]
    fn test_evaluate_bit_string() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        assert_eq!(
            utils::evaluate_bit_string(TEST_NAME, BIT_STRING_1, BIT_STRING_1.len()).unwrap(),
            BIT_STRING_1.len()
        );
        assert_eq!(
            utils::evaluate_bit_string(TEST_NAME, BIT_STRING_2, BIT_STRING_2.len()).unwrap(),
            BIT_STRING_2.len()
        );

        let mut success: bool;

        // pass empty string
        match utils::evaluate_bit_string(TEST_NAME, "", NUMBER_OF_BYTES) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);

        // pass invalid string
        match utils::evaluate_bit_string(TEST_NAME, INVALID_BIT_STRING, NUMBER_OF_BYTES) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);
    }

    #[test]
    fn test_random_numbers_file() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        // read file containing hex bytes
        let hex_bytes_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + HEX_BYTES_FILE;
        assert_eq!(
            utils::read_random_numbers(&hex_bytes_file).unwrap(),
            BIT_STRING_FROM_FILE
        );

        // read file containing bits
        let bit_string_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + BIT_STRING_FILE;
        assert_eq!(
            utils::read_random_numbers(&bit_string_file).unwrap(),
            BIT_STRING_FROM_FILE
        );

        let mut success: bool;

        // try to read file containing invalid character
        let invalid_char_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + INVALID_CHAR_IN_FILE;
        match utils::read_random_numbers(&invalid_char_file) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);

        // try to read non-existing file
        match utils::read_random_numbers(INVALID_FILE) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);
    }
}
