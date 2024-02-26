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

use anyhow::Result;

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

    // check validity of passed bit string
    if bit_string.is_empty() || bit_string.chars().any(|c| c != '0' && c != '1') {
        anyhow::bail!("Bit string is either empty or contains invalid character(s)");
    }

    let length = bit_string.len();
    log::debug!("Bit string has the length {}", length);

    // Recommended size is at least 100 bits. It is not an error but log a warning anyways
    if length < 100 {
        log::warn!(
            "Recommended size is at least 100 bits. Consider imprecision when calculating p-value"
        );
    }

    // first of all, determine the number of ones in given bit string and compute pre-test
    // proportion: #ones/length
    let count_one = bit_string.chars().filter(|&c| c == '1').count();

    let pre_test_proportion = (count_one as f64) / (length as f64);
    log::debug!(
        "Given bit string contains {} ones, pre-test proportion: {}",
        count_one,
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
    log::debug!("v_n_observed value: {}", v_n_observed);

    // finally, compute p-value with complementary error function
    let constant = pre_test_proportion * (1.0 - pre_test_proportion);
    let numerator = ((v_n_observed as f64) - 2.0 * (length as f64) * constant).abs();
    let denominator = 2.0 * (2.0 * (length as f64)).sqrt() * constant;
    log::debug!("Numerator: {}, Denominator: {}", numerator, denominator);

    let p_value = statrs::function::erf::erfc(numerator / denominator);
    log::info!("Runs: p-value of bit string is: {}", p_value);

    Ok(p_value)
}
