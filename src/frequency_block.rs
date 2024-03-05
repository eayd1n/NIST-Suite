//! This module performs the Frequency within a block Test.
//!
//! Description of test from NIST SP 800-22:
//!
//! "The focus of the test is the proportion of ones within M-bit blocks. The purpose of this test is to determine
//! whether the frequency of ones in an M-bit block is approximately M/2, as would be expected under an
//! assumption of randomness. For block size M=1, this test degenerates to test 1, the Frequency (Monobit)
//! test."

use crate::constants;
use crate::customtypes;
use crate::utils;
use anyhow::{Context, Result};

const TEST_NAME: customtypes::Test = customtypes::Test::FrequencyBlock;

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

    // capture the current time before executing the actual test
    let start_time = std::time::Instant::now();

    // check if bit string contains invalid characters
    let length = utils::evaluate_bit_string(TEST_NAME, bit_string, constants::RECOMMENDED_SIZE)
        .with_context(|| "Invalid character(s) in passed bit string detected")?;

    // check block size M for validity and get number of blocks N
    let n_blocks = evaluate_block_size(length, block_size_m).with_context(|| {
        format!(
            "{}: Either block size M or number of blocks N does not fit to defined requirements",
            TEST_NAME
        )
    })?;

    // determine the number of ones in each block. Then calculate pi_i = #ones_per_block/block_size_m
    let mut pi_i = Vec::new();
    let mut index = 0;

    for block_num in 0..n_blocks {
        let block = &bit_string[index..(index + block_size_m)];
        let count_ones = block.chars().filter(|&c| c == '1').count();
        log::trace!(
            "{}: Block {}/{}: '{}' consists of {} ones",
            TEST_NAME,
            block_num + 1,
            n_blocks,
            block,
            count_ones
        );

        pi_i.push((count_ones as f64) / (block_size_m as f64));

        index += block_size_m;
    }

    // now compute the chi_square statistics: chi_square = 4 * M * sum(p_i - 0.5)^2
    let mut observed = 0.0;

    for (index, pi) in pi_i.iter().enumerate() {
        log::trace!("pi_{}: {}", index + 1, pi);
        observed += (pi - 0.5).powf(2.0);
    }
    log::debug!("{}: Calculated observed value {}", TEST_NAME, observed);

    let chi_square = 4.0 * (block_size_m as f64) * observed;
    log::debug!(
        "{}: Chi square for given bit string: {}",
        TEST_NAME,
        chi_square
    );

    // finally, compute the p-value using the incomplete gamma function: igamc(N/2, chi_square/2)
    // Note: If we do have a perfect distribution (M/2 ones in each block), chi_square is zero
    // which is an invalid input for igamc. Return p-value of 1 then
    let p_value = if chi_square == 0.0 {
        1.0
    } else {
        statrs::function::gamma::gamma_ur((n_blocks as f64) * 0.5, chi_square * 0.5)
    };
    log::info!("{}: p-value = {}", TEST_NAME, p_value);

    // capture the current time after the test got executed and calculate elapsed time
    let end_time = std::time::Instant::now();
    let elapsed_time = end_time.duration_since(start_time).as_secs_f64();
    log::info!("{} took {:.6} seconds", TEST_NAME, elapsed_time);

    Ok(p_value)
}

/// Evaluate passed block size and return number of blocks.
///
/// # Arguments
///
/// length - Bit string length
/// block_size_m - The block size M to be evaluated
///
/// # Return
///
/// Ok(n_blocks) - Number of blocks to be processed based on block size M
/// Err(err) - Some error occured
fn evaluate_block_size(length: usize, block_size_m: usize) -> Result<usize> {
    log::trace!("frequency_block::evaluate_block_size()");

    // M should be less than bit string length but greater than (length / 100)
    if block_size_m >= length || block_size_m <= (length / constants::RECOMMENDED_SIZE) {
        anyhow::bail!(
            "{}: Choose block size as of {} < M < {}",
            TEST_NAME,
            length / constants::RECOMMENDED_SIZE,
            length
        );
    }

    // calculate number of blocks N by floor(length/block_size_m). N should be < 100
    let n_blocks = length / block_size_m;
    if n_blocks >= constants::RECOMMENDED_SIZE {
        anyhow::bail!(
            "{}: Number of blocks exceed {}: {}. Please choose a larger M",
            TEST_NAME,
            constants::RECOMMENDED_SIZE,
            n_blocks
        );
    }

    log::info!(
        "{}: Block size M: {}, number of blocks N to proceed: {}",
        TEST_NAME,
        block_size_m,
        n_blocks
    );

    Ok(n_blocks)
}
