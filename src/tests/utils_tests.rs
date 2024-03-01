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
    const INVALID_BIT_STRING: &str = "010101010101011110101010101010101X0101010101010101010101";
    const NUMBER_OF_BYTES: usize = 100;
    const RANDOM_NUMBERS_FILE: &str = "/tmp/random_numbers";
    const INVALID_FILE: &str = "/root/random_numbers";

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

    #[test]
    #[serial]
    fn test_evaluate_bit_string() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        assert_eq!(
            utils::evaluate_bit_string(BIT_STRING_1, BIT_STRING_1.len()).unwrap(),
            BIT_STRING_1.len()
        );
        assert_eq!(
            utils::evaluate_bit_string(BIT_STRING_2, BIT_STRING_2.len()).unwrap(),
            BIT_STRING_2.len()
        );

        let mut success: bool;

        // pass empty string
        match utils::evaluate_bit_string("", NUMBER_OF_BYTES) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);

        // pass invalid string
        match utils::evaluate_bit_string(INVALID_BIT_STRING, NUMBER_OF_BYTES) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);
    }

    #[test]
    #[serial]
    fn test_get_random_bytes() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        let random_bytes = match utils::get_random_bytes(NUMBER_OF_BYTES) {
            Ok(random_bytes) => random_bytes,
            Err(_) => Vec::<u8>::new(),
        };
        assert_eq!(random_bytes.len(), NUMBER_OF_BYTES);

        // pass zero bytes
        let random_bytes = match utils::get_random_bytes(0) {
            Ok(random_bytes) => random_bytes,
            Err(_) => Vec::<u8>::new(),
        };
        assert!(random_bytes.is_empty());
    }

    #[test]
    #[serial]
    fn test_random_bytes_file() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        if std::path::Path::new(RANDOM_NUMBERS_FILE).exists() {
            let _ = std::fs::remove_file(RANDOM_NUMBERS_FILE);
        }
        assert!(!std::path::Path::new(RANDOM_NUMBERS_FILE).exists());

        // first of all, generate 100 random bytes and write them to file
        let random_bytes = utils::get_random_bytes(NUMBER_OF_BYTES).unwrap();
        assert_eq!(random_bytes.len(), NUMBER_OF_BYTES);

        let _ = utils::write_random_bytes(random_bytes, RANDOM_NUMBERS_FILE);
        assert!(std::path::Path::new(RANDOM_NUMBERS_FILE).exists());

        // now read the file and get bit string
        let bit_string = utils::read_random_bytes(RANDOM_NUMBERS_FILE).unwrap();
        assert_eq!(bit_string.len(), NUMBER_OF_BYTES * 8);

        // pass empty vector to write_random_bytes
        let mut success: bool;
        match utils::write_random_bytes(Vec::<u8>::new(), RANDOM_NUMBERS_FILE) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);

        // pass invalid file to both read_random_bytes and write_random_bytes
        match utils::write_random_bytes(vec![0x00], INVALID_FILE) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);

        match utils::read_random_bytes(INVALID_FILE) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);

        // cleanup
        let _ = std::fs::remove_file(RANDOM_NUMBERS_FILE);
        assert!(!std::path::Path::new(RANDOM_NUMBERS_FILE).exists());
    }
}
