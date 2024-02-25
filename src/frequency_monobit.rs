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

use anyhow::Result;

/// Perform the Frequency Monobit Test by determining the p-value.
///
/// # Arguments
///
/// bit_string - The bit string to be tested for randomness
///
/// # Return
///
/// Ok(p-value >= 0.01) - If true, given bit string can be concluded as random. Otherwise
/// non-random
/// Err(err) - Some error occured
pub fn perform_test(bit_string: &str) -> Result<bool> {
    log::trace!("frequency_monobit::perform_test()");

    // check validity of passed bit string
    if bit_string.is_empty() || bit_string.chars().any(|c| c != '0' && c != '1') {
        anyhow::bail!("Invalid or empty bit string: '{}'", bit_string);
    }

    let length = bit_string.len();
    log::debug!("Bit string '{}' has the length {}", bit_string, length);

    // Recommended size is at least 100 bits. It is not an error but log a warning
    if length < 100 {
        log::warn!(
            "Recommended size is at least 100 bits. Consider imprecision when calculating p-value"
        );
    }

    // first of all, we need to compute the partial sum S_n. This means '1' is a +1 whereas a '0' is a -1
    let mut count_zero = 0;
    let mut count_one = 0;

    for bit in bit_string.chars() {
        match bit {
            '0' => count_zero += 1,
            '1' => count_one += 1,
            _ => {}
        };
    }
    log::info!(
        "Bit string contains {} zeros and {} ones",
        count_zero,
        count_one
    );

    let partial_sum = (count_zero - count_one) as f64;

    // now calculate observed value S_obs = |S_n| / sqrt(length)
    let observed = partial_sum.abs() / (length as f64).sqrt();
    log::info!("Observed value S_obs: {}", observed);

    // finally, compute p-value to decide whether given bit string is random or not
    // Therefore we need the complementary error function erfc(observed / sqrt(2))
    let p_value = statrs::function::erf::erfc(observed / f64::sqrt(2.0));
    log::info!("p-value of bit string is {}", p_value);

    Ok(p_value >= 0.01)
}
