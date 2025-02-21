//! This module performs the Runs test. If the Frequency test within a block test is not passed,
//! this test will NOT be executed!
//!
//! Description of test from NIST SP 800-22:
//!
//! "The focus of this test is the total number of runs in the sequence, where a run is an uninterrupted sequence
//! of identical bits. A run of length k consists of exactly k identical bits and is bounded before and after with
//! a bit of the opposite value. The purpose of the runs test is to determine whether the number of runs of
//! ones and zeros of various lengths is as expected for a random sequence. In particular, this test determines
//! whether the oscillation between such zeros and ones is too fast or too slow."

use crate::constants;
use crate::customtypes;
use crate::utils;
use anyhow::{bail, Context, Result};

const TEST_NAME: customtypes::Test = customtypes::Test::Runs;

/// Perform the "Runs" test.
///
/// # Arguments
///
/// bit_string -  The bit string to be tested for randomness
///
/// # Return
///
/// Ok(p-value) - The p-value which indicates whether randomness is given or not
/// Err(err) - Some error occured
pub fn perform_test(bit_string: &str) -> Result<f64> {
    log::trace!("runs::perform_test()");

    // capture the current time before executing the actual test
    let start_time = std::time::Instant::now();

    // check if bit string contains invalid characters
    let length = utils::evaluate_bit_string(TEST_NAME, bit_string, constants::RECOMMENDED_SIZE)
        .with_context(|| "Invalid character(s) in passed bit string detected")?
        as f64;

    // determine the number of ones in given bit string and compute pre-test proportion = #ones/length
    let pre_test_proportion = compute_pre_test_proportion(bit_string, length);

    // check whether test can be performed if requirement 2 / sqrt(length) is not satisfied
    evaluate_requirement(length, pre_test_proportion)?;

    // compute observed runs test statistics V_n(obs). Therefore compare current bit with
    // consecutive one. If not equal, add 1 to counter, otherwise do nothing
    let v_n_observed = compute_v_n_observed(bit_string);

    // finally, compute p-value with complementary error function
    let fraction = compute_fraction(pre_test_proportion, v_n_observed, length);

    let p_value = statrs::function::erf::erfc(fraction);
    log::info!("{TEST_NAME}: p-value = {p_value}");

    // capture the current time after the test got executed and calculate elapsed time
    let end_time = std::time::Instant::now();
    let elapsed_time = end_time.duration_since(start_time).as_secs_f64();
    log::info!("{TEST_NAME} took {:.6} seconds", elapsed_time);

    Ok(p_value)
}

fn compute_pre_test_proportion(bit_string: &str, length: f64) -> f64 {
    log::trace!("runs::compute_pre_test_proportion()");

    let count_ones = bit_string.chars().filter(|&c| c == '1').count() as f64;

    let pre_test_proportion = count_ones / length;
    log::debug!(
        "{TEST_NAME}: Given bit string contains {} ones and {} zeros, pre-test proportion: {}",
        count_ones,
        length - count_ones,
        pre_test_proportion
    );

    pre_test_proportion
}

fn evaluate_requirement(length: f64, pre_test_proportion: f64) -> Result<()> {
    log::trace!("runs::evaluate_requirement()");

    let tau = 2.0 / (length).sqrt();
    let requirement = (pre_test_proportion - 0.5).abs();

    if requirement >= tau {
        bail!("{TEST_NAME} is not applicable! Tau ({tau}) < Requirement ({requirement})");
    }

    Ok(())
}

fn compute_v_n_observed(bit_string: &str) -> u64 {
    log::trace!("runs::compute_v_n_observed()");

    let mut v_n_observed = 1;
    let bytes = bit_string.as_bytes();

    for bit in 0..bytes.len() - 1 {
        if bytes[bit] != bytes[bit + 1] {
            v_n_observed += 1;
        }
    }
    log::debug!("{TEST_NAME}: v_n_observed value: {v_n_observed}");

    v_n_observed
}

fn compute_fraction(pre_test_proportion: f64, v_n_observed: u64, length: f64) -> f64 {
    log::trace!("runs::compute_fraction()");

    let constant = pre_test_proportion * (1.0 - pre_test_proportion);
    let numerator = ((v_n_observed as f64) - 2.0 * length * constant).abs();
    let denominator = 2.0 * (2.0 * length).sqrt() * constant;

    log::debug!("{TEST_NAME}: Numerator: {numerator}, Denominator: {denominator}");

    numerator / denominator
}

#[cfg(test)]
mod tests {
    use crate::constants;
    use crate::logger;
    use crate::runs;
    use crate::utils;

    const LOGLEVEL: &str = "Debug";
    const BIT_STRING_NIST_1: &str = "1001101011";
    const P_VALUE_NIST_1: f64 = 0.14723225537016021;
    const BIT_STRING_NIST_2: &str = "1100100100001111110110101010001000100001011010001100001000110100110001001100011001100010100010111000";
    const P_VALUE_NIST_2: f64 = 0.5007979178870894;
    const BIT_STRING_ONLY_ZEROS: &str = "0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
    const BIT_STRING_ONLY_ONES: &str = "1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111";
    const BIT_STRING_FAST_OSCILLATION: &str = "1010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010";
    const BIT_STRING_SLOW_OSCILLATION: &str = "1011111111111111111111110111111111111111111111111101111111111111111111111111101111111111111111111110";
    const BIT_STRING_PERFECT: &str = "1100110011001100110011001100110011001100110011001100110011001100110011001100110011001100110011001100";
    const INVALID_BIT_STRING: &str = "010101111010101010101010101010a0101010101010100101010101";

    #[test]
    fn test_runs() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        assert_eq!(
            runs::perform_test(BIT_STRING_NIST_1).unwrap(),
            P_VALUE_NIST_1
        );
        assert_eq!(
            runs::perform_test(BIT_STRING_NIST_2).unwrap(),
            P_VALUE_NIST_2
        );
        assert!(runs::perform_test(BIT_STRING_FAST_OSCILLATION).unwrap() <= 0.01);
        assert!(runs::perform_test(BIT_STRING_PERFECT).unwrap() == 1.00);

        // test pi, e, sqrt(2) and sqrt(3) in their respective binary representations
        let pi_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + constants::PI_FILE;
        let pi_bit_string = utils::read_random_numbers(&pi_file).unwrap();
        assert!(runs::perform_test(&pi_bit_string).unwrap() >= 0.01);

        let e_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + constants::E_FILE;
        let e_bit_string = utils::read_random_numbers(&e_file).unwrap();
        assert!(runs::perform_test(&e_bit_string).unwrap() >= 0.01);

        let sqrt_2_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + constants::SQRT_2_FILE;
        let sqrt_2_bit_string = utils::read_random_numbers(&sqrt_2_file).unwrap();
        assert!(runs::perform_test(&sqrt_2_bit_string).unwrap() >= 0.01);

        let sqrt_3_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + constants::SQRT_3_FILE;
        let sqrt_3_bit_string = utils::read_random_numbers(&sqrt_3_file).unwrap();
        assert!(runs::perform_test(&sqrt_3_bit_string).unwrap() >= 0.01);

        let sha_3_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + constants::SHA_3_FILE;
        let sha_3_bit_string = utils::read_random_numbers(&sha_3_file).unwrap();
        assert!(runs::perform_test(&sha_3_bit_string).unwrap() >= 0.01);
    }

    #[test]
    fn test_runs_error_cases() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        // pass empty string
        assert!(runs::perform_test("").is_err());

        // pass invalid bit string
        assert!(runs::perform_test(INVALID_BIT_STRING).is_err());

        // pass bit strings which are not applicable with test
        assert!(runs::perform_test(BIT_STRING_ONLY_ZEROS).is_err());
        assert!(runs::perform_test(BIT_STRING_ONLY_ONES).is_err());
        assert!(runs::perform_test(BIT_STRING_SLOW_OSCILLATION).is_err());
    }
}
