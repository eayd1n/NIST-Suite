//! This module performs the Frequency Monobit Test.
//! If this test does not pass, the remaining tests are NOT executed (makes sense, right?)
//!
//! Description of test from NIST SP 800-22:
//!
//! "The focus of the test is the proportion of zeroes and ones for the entire sequence. The purpose of this test
//! is to determine whether the number of ones and zeros in a sequence are approximately the same as would
//! be expected for a truly random sequence. The test assesses the closeness of the fraction of ones to 1⁄2, that
//! is, the number of ones and zeroes in a sequence should be about the same. All subsequent tests depend on
//! the passing of this test."

use crate::constants;
use crate::customtypes;
use crate::utils;
use anyhow::{Context, Result};

const TEST_NAME: customtypes::Test = customtypes::Test::FrequencyMonobit;

/// Perform the "Frequency Monobit" test by determining the p-value.
///
/// # Arguments
///
/// bit_string - The bit string to be tested for randomness
///
/// # Return
///
/// Ok(p-value) - The p-value which indicates whether randomness is given or not
/// Err(err) - Some error occured
pub fn perform_test(bit_string: &str) -> Result<f64> {
    log::trace!("frequency_monobit::perform_test()");

    // capture the current time before executing the actual test
    let start_time = std::time::Instant::now();

    // check if bit string contains invalid characters
    let length = utils::evaluate_bit_string(TEST_NAME, bit_string, constants::RECOMMENDED_SIZE)
        .with_context(|| "Invalid character(s) in passed bit string detected")?
        as f64;

    // first of all, we need to compute the partial sum S_n. '1' is a +1 and '0' is a -1.
    let partial_sum = compute_partial_sum(bit_string);

    // now calculate observed value S_obs = |S_n| / sqrt(length)
    let observed = (partial_sum.abs() as f64) / length.sqrt();
    log::debug!("{TEST_NAME}: Observed value S_obs: {observed}");

    // finally, compute p-value to decide whether given bit string is random or not
    // Therefore we need the complementary error function: erfc(observed / sqrt(2))
    let p_value = statrs::function::erf::erfc(observed / std::f64::consts::SQRT_2);
    log::info!("{TEST_NAME}: p-value = {p_value}");

    // capture the current time after the test got executed and calculate elapsed time
    let end_time = std::time::Instant::now();
    let elapsed_time = end_time.duration_since(start_time).as_secs_f64();
    log::info!("{TEST_NAME} took {:.6} seconds", elapsed_time);

    Ok(p_value)
}

fn compute_partial_sum(bit_string: &str) -> i64 {
    log::trace!("frequency_monobit::compute_partial_sum()");

    let mut partial_sum: i64 = 0;

    for bit in bit_string.chars() {
        if bit == '1' {
            partial_sum += 1;
        } else {
            partial_sum -= 1;
        }
    }

    log::debug!("{TEST_NAME}: Partial Sum S_n: {partial_sum}");

    partial_sum
}

#[cfg(test)]
mod tests {
    use crate::constants;
    use crate::frequency_monobit;
    use crate::logger;
    use crate::utils;

    const LOGLEVEL: &str = "Info";
    const BIT_STRING_NIST_1: &str = "1011010101";
    const PARTIAL_SUM_1: i64 = 2;
    const P_VALUE_NIST_1: f64 = 0.5270892568655381;
    const BIT_STRING_NIST_2: &str = "1100100100001111110110101010001000100001011010001100001000110100110001001100011001100010100010111000";
    const PARTIAL_SUM_2: i64 = -16;
    const P_VALUE_NIST_2: f64 = 0.10959858340379211;
    const BIT_STRING_ONLY_ZEROS: &str = "0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
    const PARTIAL_SUM_ONLY_ZEROS: i64 = -100;
    const BIT_STRING_ONLY_ONES: &str = "1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111";
    const PARTIAL_SUM_ONLY_ONES: i64 = 100;
    const BIT_STRING_NON_RANDOM: &str = "10101010101111111111111";
    const BIT_STRING_RANDOM: &str = "11001001000011111101101010100010001000010110100011000010001101001100010011000110011000101000101110001100100100001111110110101010001000100001011010001100001000110100110001001100011001100010100010111000";
    const BIT_STRING_PERFECT: &str = "0000000000000000000000000000000000000000000000000011111111111111111111111111111111111111111111111111";
    const PARTIAL_SUM_PERFECT: i64 = 0;
    const INVALID_BIT_STRING: &str = "010101111010101010101010101010a0101010101010100101010101";

    #[test]
    fn test_partial_sum_computation() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        assert_eq!(
            frequency_monobit::compute_partial_sum(BIT_STRING_NIST_1),
            PARTIAL_SUM_1
        );
        assert_eq!(
            frequency_monobit::compute_partial_sum(BIT_STRING_NIST_2),
            PARTIAL_SUM_2
        );
        assert_eq!(
            frequency_monobit::compute_partial_sum(BIT_STRING_ONLY_ZEROS),
            PARTIAL_SUM_ONLY_ZEROS
        );
        assert_eq!(
            frequency_monobit::compute_partial_sum(BIT_STRING_ONLY_ONES),
            PARTIAL_SUM_ONLY_ONES
        );
        assert_eq!(
            frequency_monobit::compute_partial_sum(BIT_STRING_PERFECT),
            PARTIAL_SUM_PERFECT
        );
    }

    #[test]
    fn test_frequency_monobit() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        assert_eq!(
            frequency_monobit::perform_test(BIT_STRING_NIST_1).unwrap(),
            P_VALUE_NIST_1
        );
        assert_eq!(
            frequency_monobit::perform_test(BIT_STRING_NIST_2).unwrap(),
            P_VALUE_NIST_2
        );
        assert!(frequency_monobit::perform_test(BIT_STRING_ONLY_ZEROS).unwrap() < 0.01);
        assert!(frequency_monobit::perform_test(BIT_STRING_ONLY_ONES).unwrap() < 0.01);
        assert_eq!(
            frequency_monobit::perform_test(BIT_STRING_ONLY_ZEROS).unwrap(),
            frequency_monobit::perform_test(BIT_STRING_ONLY_ONES).unwrap()
        );
        assert!(frequency_monobit::perform_test(BIT_STRING_NON_RANDOM).unwrap() < 0.01);
        assert!(frequency_monobit::perform_test(BIT_STRING_RANDOM).unwrap() >= 0.01);
        assert!(frequency_monobit::perform_test(BIT_STRING_PERFECT).unwrap() == 1.00);

        // test pi, e, sqrt(2) and sqrt(3) in their respective binary representations
        let pi_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + constants::PI_FILE;
        let pi_bit_string = utils::read_random_numbers(&pi_file).unwrap();
        assert!(frequency_monobit::perform_test(&pi_bit_string).unwrap() >= 0.01);

        let e_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + constants::E_FILE;
        let e_bit_string = utils::read_random_numbers(&e_file).unwrap();
        assert!(frequency_monobit::perform_test(&e_bit_string).unwrap() >= 0.01);

        let sqrt_2_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + constants::SQRT_2_FILE;
        let sqrt_2_bit_string = utils::read_random_numbers(&sqrt_2_file).unwrap();
        assert!(frequency_monobit::perform_test(&sqrt_2_bit_string).unwrap() >= 0.01);

        let sqrt_3_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + constants::SQRT_3_FILE;
        let sqrt_3_bit_string = utils::read_random_numbers(&sqrt_3_file).unwrap();
        assert!(frequency_monobit::perform_test(&sqrt_3_bit_string).unwrap() >= 0.01);

        let sha_3_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + constants::SHA_3_FILE;
        let sha_3_bit_string = utils::read_random_numbers(&sha_3_file).unwrap();
        assert!(frequency_monobit::perform_test(&sha_3_bit_string).unwrap() >= 0.01);
    }

    #[test]
    fn test_frequency_monobit_error_cases() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        // pass empty string
        assert!(frequency_monobit::perform_test("").is_err());

        // pass invalid bit string
        assert!(frequency_monobit::perform_test(INVALID_BIT_STRING).is_err());
    }
}
