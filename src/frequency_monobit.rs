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

/// Perform the Frequency Monobit Test by determining the p-value.
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
    let mut partial_sum: i32 = 0;

    for bit in bit_string.chars() {
        if bit == '1' {
            partial_sum += 1;
        } else {
            partial_sum -= 1;
        }
    }
    log::debug!("{}: Partial Sum S_n: {}", TEST_NAME, partial_sum);

    // now calculate observed value S_obs = |S_n| / sqrt(length)
    let observed = (partial_sum.abs() as f64) / length.sqrt();
    log::debug!("{}: Observed value S_obs: {}", TEST_NAME, observed);

    // finally, compute p-value to decide whether given bit string is random or not
    // Therefore we need the complementary error function: erfc(observed / sqrt(2))
    let p_value = statrs::function::erf::erfc(observed / std::f64::consts::SQRT_2);
    log::info!("{}: p-value = {}", TEST_NAME, p_value);

    // capture the current time after the test got executed and calculate elapsed time
    let end_time = std::time::Instant::now();
    let elapsed_time = end_time.duration_since(start_time).as_secs_f64();
    log::info!("{} took {:.6} seconds", TEST_NAME, elapsed_time);

    Ok(p_value)
}
