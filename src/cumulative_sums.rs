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

/// Perform the Cumulative Sums Test.
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

    // now compute the particular sums and determine the maximum sum. Instead of adding +1 for a
    // '1' and -1 for a '0', just determine the number of ones and zeros and calculate the
    // difference
    let mut max_sum_z = 0;

    for i in 0..length {
        let num_bits = i + 1;
        let block = &new_bit_string[0..num_bits];

        let count_zeros = block.chars().filter(|&c| c == '0').count();
        let count_ones = block.len() - count_zeros;

        let current_sum = if count_zeros >= count_ones {
            count_zeros - count_ones
        } else {
            count_ones - count_zeros
        };

        if current_sum > max_sum_z {
            max_sum_z = current_sum;
        }
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
