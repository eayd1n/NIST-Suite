//! This module performs the Binary Matrix Rank Test.
//!
//! Description of test from NIST SP 800-22:
//!
//! "The focus of the test is the rank of disjoint sub-matrices of the entire sequence. The purpose of this test is
//! to check for linear dependence among fixed length substrings of the original sequence."

use anyhow::Result;

/// Perform the Binary Matrix Rank Test by determining the p-value.
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
    log::trace!("binary_matrix_rank::perform_test()");

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

    Ok(p_value >= 0.01)
}
