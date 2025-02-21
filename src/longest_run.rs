//! This module performs the Longest Run of Ones in a Block test. For this test, it is crucial to pass
//! least 128 bit!
//!
//! Description of test from NIST SP 800-22:
//!
//! "The focus of the test is the longest run of ones within M-bit blocks. The purpose of this test is to
//! determine whether the length of the longest run of ones within the tested sequence is consistent with the
//! length of the longest run of ones that would be expected in a random sequence. Note that an irregularity in
//! the expected length of the longest run of ones implies that there is also an irregularity in the expected
//! length of the longest run of zeroes. Therefore, only a test for ones is necessary."

use crate::constants;
use crate::customtypes;
use crate::utils;
use anyhow::{Context, Result};
use std::collections::BTreeMap;

const TEST_NAME: customtypes::Test = customtypes::Test::LongestRun;

/// Perform the "Longest Run of Ones in a Block" test.
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
    log::trace!("longest_run::perform_test()");

    // capture the current time before executing the actual test
    let start_time = std::time::Instant::now();

    // check if bit string contains invalid characters
    let length = utils::evaluate_bit_string(TEST_NAME, bit_string, constants::MIN_LENGTH)
        .with_context(|| "Invalid character(s) in passed bit string detected")?;

    // evaluate bit string length and determine longest run configuration
    let config = get_longest_run_config(length)
        .with_context(|| format!("{TEST_NAME}: Failed to retrieve longest run configuration"))?;

    // determine the number of runs per block and calculate v_i. A "longest" run is defined as the
    // maximum number of consecutive ones in a block, e.g., "110010111" has the longest run as of 3
    let mut counts: BTreeMap<i32, i32> = BTreeMap::new();

    for block_num in 0..config.number_of_blocks {
        let start_index = block_num * config.block_size;
        let end_index = (block_num + 1) * config.block_size;
        let block = &bit_string[start_index..end_index];
        let max_consecutive_ones = count_max_consecutive_ones(block);

        *counts.entry(max_consecutive_ones).or_insert(0) += 1;
    }

    log::debug!("{TEST_NAME}: Number of runs before merge: {:?}", counts);
    let vi_counts = calculate_vi_values(counts, config.thresholds);
    log::debug!("{TEST_NAME}: Number of runs after merge: {:?}", vi_counts);

    // Now we need to compute chi_square value
    let mut chi_square = 0.0;

    // iterate over vi_values and pi_values at the same time because both have same size
    for ((_, vi_value), &pi_value) in vi_counts.iter().zip(config.pi_values.iter()) {
        log::trace!(
            "{TEST_NAME}: Current vi_value: {}, current pi_value: {}",
            *vi_value,
            pi_value
        );

        let constant = (config.number_of_blocks as f64) * pi_value;
        chi_square += ((*vi_value as f64) - constant).powf(2.0) / constant;
    }
    log::debug!("{TEST_NAME}: Value of chi_square: {chi_square}");

    // finally compute p-value with the incomplete gamma function: igamc(K/2, chi_square/2)
    let p_value = statrs::function::gamma::gamma_ur(
        ((config.pi_values.len() as f64) - 1.0) * 0.5,
        chi_square * 0.5,
    );
    log::info!("{TEST_NAME}: p-value = {p_value}");

    // capture the current time after the test got executed and calculate elapsed time
    let end_time = std::time::Instant::now();
    let elapsed_time = end_time.duration_since(start_time).as_secs_f64();
    log::info!("{TEST_NAME} took {:.6} seconds", elapsed_time);

    Ok(p_value)
}

fn get_longest_run_config(length: usize) -> Result<customtypes::LongestRunConfig<'static>> {
    log::trace!("longest_run::get_longest_run_config()");

    // it is crucial to have at least 128 bit passed for the test
    if length < constants::MIN_LENGTH {
        anyhow::bail!(
            "{TEST_NAME}: Bit string needs at least {} bits! Actual length: {length}",
            constants::MIN_LENGTH
        );
    }

    // depending on length of bit string, choose the correct value for M (number of bits per
    // block), N (number of blocks), thresholds (min, max) and the pre-computed pi_values
    let config: customtypes::LongestRunConfig;

    if (constants::MIN_LENGTH..constants::MID_LENGTH).contains(&length) {
        config = customtypes::LongestRunConfig::create(
            constants::MIN_SIZE_M,
            constants::MIN_SIZE_N,
            constants::MIN_THRESHOLDS,
            &constants::MIN_PI_VALUES,
        );
    } else if (constants::MID_LENGTH..constants::MAX_LENGTH).contains(&length) {
        config = customtypes::LongestRunConfig::create(
            constants::MID_SIZE_M,
            constants::MID_SIZE_N,
            constants::MID_THRESHOLDS,
            &constants::MID_PI_VALUES,
        );
    } else {
        config = customtypes::LongestRunConfig::create(
            constants::MAX_SIZE_M,
            constants::MAX_SIZE_N,
            constants::MAX_THRESHOLDS,
            &constants::MAX_PI_VALUES,
        );
    }
    log::debug!("{TEST_NAME}: Configured following values: {:?}", config);

    Ok(config)
}

fn count_max_consecutive_ones(block: &str) -> i32 {
    log::trace!("longest_run::count_max_consecutive_ones()");

    let mut max_count = 0;
    let mut current_count = 0;

    for bit in block.chars() {
        if bit == '1' {
            current_count += 1;
            max_count = max_count.max(current_count);
        } else {
            current_count = 0
        }
    }

    log::trace!("{TEST_NAME}: Block '{block}', longest run of ones: {max_count}");
    max_count
}

fn calculate_vi_values(
    run_counts: BTreeMap<i32, i32>,
    thresholds: (i32, i32),
) -> BTreeMap<i32, i32> {
    log::trace!("longest_run::calculate_vi_values()");

    let mut vi_counts: BTreeMap<i32, i32> = BTreeMap::new();

    for (&key, &value) in &run_counts {
        if key <= thresholds.0 {
            *vi_counts.entry(thresholds.0).or_insert(0) += value;
        } else if key >= thresholds.1 {
            *vi_counts.entry(thresholds.1).or_insert(0) += value;
        } else {
            *vi_counts.entry(key).or_insert(0) += value;
        }
    }

    // Iterate from the minimum threshold to the maximum threshold
    for threshold in thresholds.0..=thresholds.1 {
        // If there were no counts for the current threshold, insert a zero count for the current
        // threshold
        vi_counts.entry(threshold).or_insert(0);
    }

    vi_counts
}

#[cfg(test)]
mod tests {
    use crate::constants;
    use crate::logger;
    use crate::longest_run;
    use crate::utils;

    const LOGLEVEL: &str = "Debug";
    const BIT_STRING_NIST_1: &str = "11001100000101010110110001001100111000000000001001001101010100010001001111010110100000001101011111001100111001101101100010110010";
    const P_VALUE_NIST_1: f64 = 0.18060931823971144;
    const BIT_STRING_ONLY_ZEROS: &str = "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
    const BIT_STRING_ONLY_ONES: &str = "11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111";
    const BIT_STRING_RANDOM: &str = "10000000100000001000000010000000110000001100000011000000110000001110000011100000111000001110000011110000111100001111000011110000";
    const BIT_STRING_NON_RANDOM: &str = "10000000010000000000010000000000001000000000001000000000001000000000000100000000000000000010000000000010000000000111111111111111";
    const INVALID_BIT_STRING: &str = "1100110000010101011011000100110011100000000000100100110101010001000100a111010110100000001101011111001100111001101101100010110010";

    #[test]
    fn test_longest_run() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        assert_eq!(
            longest_run::perform_test(BIT_STRING_NIST_1).unwrap(),
            P_VALUE_NIST_1
        );
        assert!(longest_run::perform_test(BIT_STRING_ONLY_ZEROS).unwrap() < 0.01);
        assert!(longest_run::perform_test(BIT_STRING_ONLY_ONES).unwrap() < 0.01);
        assert!(longest_run::perform_test(BIT_STRING_RANDOM).unwrap() >= 0.01);
        assert!(longest_run::perform_test(BIT_STRING_NON_RANDOM).unwrap() < 0.01);

        // test pi, e, sqrt(2) and sqrt(3) in their respective binary representations
        let pi_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + constants::PI_FILE;
        let pi_bit_string = utils::read_random_numbers(&pi_file).unwrap();
        assert!(longest_run::perform_test(&pi_bit_string).unwrap() >= 0.01);

        let e_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + constants::E_FILE;
        let e_bit_string = utils::read_random_numbers(&e_file).unwrap();
        assert!(longest_run::perform_test(&e_bit_string).unwrap() >= 0.01);

        let sqrt_2_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + constants::SQRT_2_FILE;
        let sqrt_2_bit_string = utils::read_random_numbers(&sqrt_2_file).unwrap();
        assert!(longest_run::perform_test(&sqrt_2_bit_string).unwrap() >= 0.01);

        let sqrt_3_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + constants::SQRT_3_FILE;
        let sqrt_3_bit_string = utils::read_random_numbers(&sqrt_3_file).unwrap();
        assert!(longest_run::perform_test(&sqrt_3_bit_string).unwrap() >= 0.01);

        let sha_3_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + constants::SHA_3_FILE;
        let sha_3_bit_string = utils::read_random_numbers(&sha_3_file).unwrap();
        assert!(longest_run::perform_test(&sha_3_bit_string).unwrap() >= 0.01);
    }

    #[test]
    fn test_longest_run_error_cases() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        // pass empty string
        assert!(longest_run::perform_test("").is_err());

        // pass invalid bit string
        assert!(longest_run::perform_test(INVALID_BIT_STRING).is_err());
    }
}
