#[cfg(test)]
mod tests {
    use crate::logger;
    use crate::non_overlapping_template;
    use crate::utils;
    use serial_test::serial;

    const LOGLEVEL: &str = "Trace";
    const BIT_STRING_1: &str = "10100100101110010110"; // example from NIST Paper. p-value should be 0.344154
    const INVALID_BIT_STRING: &str = "010101111010101010101010101010a0101010101010100101010101";

    #[test]
    #[serial]
    fn test_non_overlapping_template() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        assert!(non_overlapping_template::perform_test(BIT_STRING_1, 3).unwrap() >= 0.01);
    }

    #[test]
    #[serial]
    fn test_non_overlapping_template_error_cases() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        let mut success: bool;

        // pass empty string
        match non_overlapping_template::perform_test("", 3) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);

        // pass invalid bit string
        match non_overlapping_template::perform_test(INVALID_BIT_STRING, 6) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);

        // pass invalid template length sizes
        match non_overlapping_template::perform_test(BIT_STRING_1, 0) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);

        match non_overlapping_template::perform_test(INVALID_BIT_STRING, 11) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);
    }
}
