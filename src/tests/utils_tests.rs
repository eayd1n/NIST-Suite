#[cfg(test)]
mod tests {
    use crate::logger;
    use crate::utils;
    use serial_test::serial;

    const LOGLEVEL: &str = "Trace";
    static RANDOM_BYTES_1: [u8; 6] = [0xab, 0x00, 0xde, 0xd6, 0xf3, 0xc0];
    static RANDOM_BYTES_2: [u8; 9] = [0x00, 0xFF, 0xDE, 0x89, 0xC0, 0x3D, 0xA6, 0xC2, 0xB5];
    const BIT_STRING_1: &str = "101010110000000011011110110101101111001111000000";
    const BIT_STRING_2: &str =
        "000000001111111111011110100010011100000000111101101001101100001010110101";
    const INVALID_BIT_STRING: &str = "010101010101011110101010101010101X0101010101010101010101";
    const NUMBER_OF_BYTES: usize = 100;
    const RANDOM_NUMBERS_FILE: &str = "/tmp/random_numbers";
    const INVALID_FILE: &str = "/root/random_numbers";

    #[test]
    #[serial]
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
        let write_random_bytes = utils::get_random_bytes(NUMBER_OF_BYTES).unwrap();
        assert_eq!(write_random_bytes.len(), NUMBER_OF_BYTES);

        let _ = utils::write_random_bytes(write_random_bytes.clone(), RANDOM_NUMBERS_FILE);
        assert!(std::path::Path::new(RANDOM_NUMBERS_FILE).exists());

        // now read the file to get the random numbers
        let read_random_bytes = utils::read_random_bytes(RANDOM_NUMBERS_FILE).unwrap();
        assert_eq!(read_random_bytes, write_random_bytes);

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
