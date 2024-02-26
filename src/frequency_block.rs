//! This module performs the Frequency within a block Test.
//!
//! Description of test from NIST SP 800-22:
//!
//! "The focus of the test is the proportion of ones within M-bit blocks. The purpose of this test is to determine
//! whether the frequency of ones in an M-bit block is approximately M/2, as would be expected under an
//! assumption of randomness. For block size M=1, this test degenerates to test 1, the Frequency (Monobit)
//! test."

use anyhow::Result;

const RECOMMENDED_SIZE: usize = 100;

/// Perform the Frequncy within a block test.
///
/// # Arguments
///
/// bit_string - The bit string to be tested for randomness
/// block_size_m - Divide the bit string into equal blocks of size M
///
/// # Return
///
/// Ok(p-value) - The p-value which indicates whether randomness is given or not
/// Err(err) - Some error occured
pub fn perform_test(bit_string: &str, block_size_m: usize) -> Result<f64> {
    log::trace!("frequency_block::perform_test()");

    // check validity of passed bit string
    if bit_string.is_empty() || bit_string.chars().any(|c| c != '0' && c != '1') {
        anyhow::bail!("Bit string is either empty or contains invalid character(s)");
    }

    // check validity of block size M. M should be less than bit string length but greater than
    // (length / 100)
    let length = bit_string.len();

    if block_size_m >= length || block_size_m <= (length / RECOMMENDED_SIZE) {
        anyhow::bail!(
            "Choose block size as of {} < M < {}",
            length / RECOMMENDED_SIZE,
            length
        );
    }

    log::debug!(
        "Bit string has the length = {} and block size M = {}",
        length,
        block_size_m
    );

    // Recommended size is at least 100 bits. It is not an error but log a warning anyways
    if length < RECOMMENDED_SIZE {
        log::warn!(
            "Recommended size is at least 100 bits. Consider imprecision when calculating p-value"
        );
    }

    // calculate number of blocks N by floor(length/block_size_m). N should be < 100
    let n_blocks = length / block_size_m;
    if n_blocks >= RECOMMENDED_SIZE {
        anyhow::bail!(
            "Number of blocks exceed 100: {}. Please choose a larger M",
            n_blocks
        );
    }

    log::info!("Number of blocks N to proceed: {}", n_blocks);

    // determine the number of ones in each block. Then calculate pi_i = #ones_per_block/block_size_m
    let mut pi_i = Vec::new();
    let mut index = 0;

    for block_num in 0..n_blocks {
        // get the current block
        let block = &bit_string[index..(index + block_size_m)];
        let ones_count = block.chars().filter(|&c| c == '1').count();
        log::trace!(
            "Block {}/{}: '{}' consists of {} ones",
            block_num + 1,
            n_blocks,
            block,
            ones_count
        );

        pi_i.push((ones_count as f64) / (block_size_m as f64));

        index += block_size_m;
    }

    // now compute the chi_square statistics: chi_square = 4 * M * sum(p_i - 0.5)^2
    let mut observed = 0.0;

    for (index, pi) in pi_i.iter().enumerate() {
        log::trace!("pi_{}: {}", index + 1, pi);
        observed += (pi - 0.5).powf(2.0);
    }
    log::debug!("Calculated observed value {}", observed);

    let chi_square = 4.0 * (block_size_m as f64) * observed;
    log::debug!("Chi square for given bit string: {}", chi_square);

    // finally, compute the p-value using the incomplete gamma function: igamc(N/2, chi_square/2)
    let p_value = statrs::function::gamma::gamma_ur((n_blocks as f64) / 2.0, chi_square / 2.0);
    log::info!(
        "Frequency Within a Block: p-value of bit string is {}",
        p_value
    );

    Ok(p_value)
}
