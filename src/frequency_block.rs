//! This module performs the Frequency within a block Test.

use anyhow::Result;

pub fn perform_test(bit_string: &str) -> Result<bool> {
    log::trace!("frequency_block::perform_test()");

    // check validity of passed bit string
    if bit_string.is_empty() || bit_string.chars().any(|c| c != '0' && c != '1') {
        anyhow::bail!("Invalid or empty bit string: '{}'", bit_string);
    }

    let length = bit_string.len();
    log::info!("Bit string '{}' has the length {}", bit_string, length);

    // Recommended size is at least 100 bits. It is not an error but log a warning
    if length < 100 {
        log::warn!("Recommended size is at least 100 bits. Consider imprecision when calculating p-value");
    }

     Ok(true)
}
