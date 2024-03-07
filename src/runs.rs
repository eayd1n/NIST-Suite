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
use anyhow::{Context, Result};

const TEST_NAME: customtypes::Test = customtypes::Test::Runs;

/// Perform the Runs test.
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
    let count_ones = bit_string.chars().filter(|&c| c == '1').count() as f64;

    let pre_test_proportion = count_ones / length;
    log::debug!(
        "{}: Given bit string contains {} ones and {} zeros, pre-test proportion: {}",
        TEST_NAME,
        count_ones,
        length - count_ones,
        pre_test_proportion
    );

    // compute observed runs test statistics V_n(obs). Therefore compare current bit with
    // consecutive one. If not equal, add 1 to counter, otherwise do nothing
    let mut v_n_observed = 1;
    let bytes = bit_string.as_bytes();

    for bit in 0..bytes.len() - 1 {
        if bytes[bit] != bytes[bit + 1] {
            v_n_observed += 1;
        }
    }
    log::debug!("{}: v_n_observed value: {}", TEST_NAME, v_n_observed);

    // finally, compute p-value with complementary error function
    let constant = pre_test_proportion * (1.0 - pre_test_proportion);
    let numerator = ((v_n_observed as f64) - 2.0 * length * constant).abs();
    let denominator = 2.0 * (2.0 * length).sqrt() * constant;
    log::debug!(
        "{}: Numerator: {}, Denominator: {}",
        TEST_NAME,
        numerator,
        denominator
    );

    let p_value = statrs::function::erf::erfc(numerator / denominator);
    log::info!("{}: p-value = {}", TEST_NAME, p_value);

    // capture the current time after the test got executed and calculate elapsed time
    let end_time = std::time::Instant::now();
    let elapsed_time = end_time.duration_since(start_time).as_secs_f64();
    log::info!("{} took {:.6} seconds", TEST_NAME, elapsed_time);

    Ok(p_value)
}
