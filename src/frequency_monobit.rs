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

use crate::utils;
use anyhow::Result;

const RECOMMENDED_SIZE: usize = 100;

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

    let length = utils::evaluate_bit_string(bit_string, RECOMMENDED_SIZE)?;

    // first of all, we need to compute the partial sum S_n. This is the difference between #ones and #zeroes
    let count_zero = bit_string.chars().filter(|&c| c == '0').count();
    let count_one = length - count_zero;

    log::info!(
        "Bit string contains {} zeros and {} ones",
        count_zero,
        count_one
    );

    let partial_sum = if count_zero >= count_one {
        (count_zero - count_one) as f64
    } else {
        (count_one - count_zero) as f64
    };

    // now calculate observed value S_obs = |S_n| / sqrt(length)
    let observed = partial_sum / (length as f64).sqrt();
    log::debug!("Observed value S_obs: {}", observed);

    // finally, compute p-value to decide whether given bit string is random or not
    // Therefore we need the complementary error function: erfc(observed / sqrt(2))
    let p_value = statrs::function::erf::erfc(observed / f64::sqrt(2.0));
    log::info!("Frequency Monobit: p-value of bit string is {}", p_value);

    Ok(p_value)
}
