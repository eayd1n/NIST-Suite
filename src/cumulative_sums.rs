//! This module performs the Cumulative Sums (Cusums) Test.
//!
//! Description of test from NIST SP 800-22:
//!
//! "The focus of this test is the maximal excursion (from zero) of the random walk defined by the cumulative
//! sum of adjusted (-1, +1) digits in the sequence. The purpose of the test is to determine whether the
//! cumulative sum of the partial sequences occurring in the tested sequence is too large or too small relative
//! to the expected behavior of that cumulative sum for random sequences. This cumulative sum may be
//! considered as a random walk. For a random sequence, the excursions of the random walk should be near
//! zero. For certain types of non-random sequences, the excursions of this random walk from zero will be
//! large."

use crate::constants;
use crate::customtypes;
use crate::utils;
use anyhow::{Context, Result};
use statrs::distribution::ContinuousCDF;

const TEST_NAME: customtypes::Test = customtypes::Test::CumulativeSums;

/// Perform the "Cumulative Sums" test.
///
/// # Arguments
///
/// bit_string - The bit string to be tested for randomness
/// mode - A switch to process forward (mode = 0) or backward (mode = 1) through sequence
///
/// # Return
///
/// Ok(p-value) - The p-value which indicates whether randomness is given or not
/// Err(err) - Some error occured
pub fn perform_test(bit_string: &str, mode: customtypes::Mode) -> Result<f64> {
    log::trace!("cumulative_sums::perform_test()");

    // capture the current time before executing the actual test
    let start_time = std::time::Instant::now();

    // check if bit string contains invalid characters
    let length = utils::evaluate_bit_string(TEST_NAME, bit_string, constants::RECOMMENDED_SIZE)
        .with_context(|| "Invalid character(s) in passed bit string detected")?;

    // Create cumulative sums depending on chosen mode
    // In "Forward" mode, the bit string remains unchanged.
    // In "Backward" mode, just revert the bit string
    let new_bit_string = if mode == customtypes::Mode::Forward {
        bit_string.to_string()
    } else {
        bit_string.chars().rev().collect::<String>()
    };

    // now compute the particular sums and determine the maximum sum. '1' is a +1 whereas '0' is a
    // -1
    let mut current_sum: i64 = 0;
    let mut max_sum_z = 0;

    for bit in new_bit_string.chars() {
        if bit == '1' {
            current_sum += 1;
        } else {
            current_sum -= 1;
        }

        max_sum_z = max_sum_z.max(current_sum.abs());
    }
    log::debug!(
        "{}: Determined maximum value z of cumulative sums: {}",
        TEST_NAME,
        max_sum_z
    );

    // compute lower and upper limits for the sums before generating p-value
    let upper_limit = (((length as f64) / (max_sum_z as f64) - 1.0) * 0.25) as i64;
    let lower_limit_1 = ((-1.0 * (length as f64) / (max_sum_z as f64) + 1.0) * 0.25) as i64;
    let lower_limit_2 = ((-1.0 * (length as f64) / (max_sum_z as f64) - 3.0) * 0.25) as i64;
    log::debug!(
        "{}: Upper limit: {}, Lower Limit 1: {}, Lower Limit 2: {}",
        TEST_NAME,
        upper_limit,
        lower_limit_1,
        lower_limit_2
    );

    // finally, compute p-value with the standard normal cumulative probability distribution
    // function
    let mut sum_1 = 0.0;
    let mut sum_2 = 0.0;
    let normal = statrs::distribution::Normal::new(0.0, 1.0).unwrap();
    let denominator = (length as f64).sqrt();

    // we do have two sums to generate to get the p-value in the end
    for k in lower_limit_1..=upper_limit {
        let numerator_1 = (4.0 * (k as f64) + 1.0) * (max_sum_z as f64);
        let numerator_2 = (4.0 * (k as f64) - 1.0) * (max_sum_z as f64);

        sum_1 += normal.cdf(numerator_1 / denominator) - normal.cdf(numerator_2 / denominator);
        log::trace!(
            "{}: Value of sum in first loop for k = {}: {}",
            TEST_NAME,
            k,
            sum_1
        );
    }

    for k in lower_limit_2..=upper_limit {
        let numerator_1 = (4.0 * (k as f64) + 3.0) * (max_sum_z as f64);
        let numerator_2 = (4.0 * (k as f64) + 1.0) * (max_sum_z as f64);

        sum_2 += normal.cdf(numerator_1 / denominator) - normal.cdf(numerator_2 / denominator);
        log::trace!(
            "{}: Value of sum in second loop for k = {}: {}",
            TEST_NAME,
            k,
            sum_2
        );
    }

    let p_value = 1.0 - sum_1 + sum_2;
    log::info!("{}: p-value = {} ('{:?}' Mode)", TEST_NAME, p_value, mode);

    // capture the current time after the test got executed and calculate elapsed time
    let end_time = std::time::Instant::now();
    let elapsed_time = end_time.duration_since(start_time).as_secs_f64();
    log::info!("{} took {:.6} seconds", TEST_NAME, elapsed_time);

    Ok(p_value)
}

#[cfg(test)]
mod tests {
    use crate::cumulative_sums;
    use crate::customtypes;
    use crate::logger;
    use crate::utils;

    const LOGLEVEL: &str = "Debug";
    const BIT_STRING_NIST_1: &str = "1011010111";
    const P_VALUE_NIST_1: f64 = 0.4116586191729081;
    const BIT_STRING_NIST_2: &str = "1100100100001111110110101010001000100001011010001100001000110100110001001100011001100010100010111000";
    const P_VALUE_NIST_2_FORWARD: f64 = 0.2191939934949785;
    const P_VALUE_NIST_2_BACKWARD: f64 = 0.11486621529731965;
    const BIT_STRING_ONLY_ONES: &str = "1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111";
    const BIT_STRING_ONLY_ZEROS: &str = "0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
    const INVALID_BIT_STRING: &str = "1100110000010101011011000100110011100000000000100100110101010001000100a111010110100000001101011111001100111001101101100010110010";
    const PI_FILE: &str = "/src/testdata/data.pi";
    const E_FILE: &str = "/src/testdata/data.e";
    const SQRT_2_FILE: &str = "/src/testdata/data.sqrt2";
    const SQRT_3_FILE: &str = "/src/testdata/data.sqrt3";
    const SHA_3_FILE: &str = "/src/testdata/data.sha3";

    #[test]
    fn test_cumulative_sums() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        assert_eq!(
            cumulative_sums::perform_test(BIT_STRING_NIST_1, customtypes::Mode::Forward).unwrap(),
            P_VALUE_NIST_1
        );
        assert_eq!(
            cumulative_sums::perform_test(BIT_STRING_NIST_1, customtypes::Mode::Backward).unwrap(),
            P_VALUE_NIST_1
        );
        assert_eq!(
            cumulative_sums::perform_test(BIT_STRING_NIST_2, customtypes::Mode::Forward).unwrap(),
            P_VALUE_NIST_2_FORWARD
        );
        assert_eq!(
            cumulative_sums::perform_test(BIT_STRING_NIST_2, customtypes::Mode::Backward).unwrap(),
            P_VALUE_NIST_2_BACKWARD
        );
        assert!(
            cumulative_sums::perform_test(BIT_STRING_ONLY_ONES, customtypes::Mode::Forward)
                .unwrap()
                <= 0.01
        );
        assert!(
            cumulative_sums::perform_test(BIT_STRING_ONLY_ONES, customtypes::Mode::Backward)
                .unwrap()
                <= 0.01
        );
        assert!(
            cumulative_sums::perform_test(BIT_STRING_ONLY_ZEROS, customtypes::Mode::Forward)
                .unwrap()
                <= 0.01
        );
        assert!(
            cumulative_sums::perform_test(BIT_STRING_ONLY_ZEROS, customtypes::Mode::Backward)
                .unwrap()
                <= 0.01
        );
        assert_eq!(
            cumulative_sums::perform_test(BIT_STRING_ONLY_ZEROS, customtypes::Mode::Forward)
                .unwrap(),
            cumulative_sums::perform_test(BIT_STRING_ONLY_ONES, customtypes::Mode::Backward)
                .unwrap()
        );

        // test pi, e, sqrt(2) and sqrt(3) in their respective binary representations
        let pi_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + PI_FILE;
        let pi_bit_string = utils::read_random_numbers(&pi_file).unwrap();
        assert!(
            cumulative_sums::perform_test(&pi_bit_string, customtypes::Mode::Forward).unwrap()
                >= 0.01
        );
        assert!(
            cumulative_sums::perform_test(&pi_bit_string, customtypes::Mode::Backward).unwrap()
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
            cumulative_sums::perform_test(&e_bit_string, customtypes::Mode::Forward).unwrap()
                >= 0.01
        );
        assert!(
            cumulative_sums::perform_test(&e_bit_string, customtypes::Mode::Backward).unwrap()
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
            cumulative_sums::perform_test(&sqrt_2_bit_string, customtypes::Mode::Forward).unwrap()
                >= 0.01
        );
        assert!(
            cumulative_sums::perform_test(&sqrt_2_bit_string, customtypes::Mode::Backward).unwrap()
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
            cumulative_sums::perform_test(&sqrt_3_bit_string, customtypes::Mode::Forward).unwrap()
                >= 0.01
        );
        assert!(
            cumulative_sums::perform_test(&sqrt_3_bit_string, customtypes::Mode::Backward).unwrap()
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
            cumulative_sums::perform_test(&sha_3_bit_string, customtypes::Mode::Forward).unwrap()
                >= 0.01
        );
        assert!(
            cumulative_sums::perform_test(&sha_3_bit_string, customtypes::Mode::Backward).unwrap()
                >= 0.01
        );
    }

    #[test]
    fn test_cumulative_sums_error_cases() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        // pass empty string
        assert!(cumulative_sums::perform_test("", customtypes::Mode::Backward).is_err());

        // pass invalid bit string
        assert!(cumulative_sums::perform_test(INVALID_BIT_STRING, customtypes::Mode::Forward).is_err());
    }
}
