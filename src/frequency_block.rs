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
        log::warn!(
            "Recommended size is at least 100 bits. Consider imprecision when calculating p-value"
        );
    }

    // calculate block size M and number of blocks N by just using floor(sqrt(length))
    let block_size_m = ((length as f64).sqrt()).floor() as usize;
    let n_blocks = ((length as f64).sqrt()).floor() as usize;
    log::info!(
        "Block size M: {}, Number of blocks N: {}",
        block_size_m,
        n_blocks
    );

    // determine the number of ones in each block. Then calculate pi_i = #ones_per_block/block_size_m
    let mut pi_i: Vec<f64> = Vec::new();
    let mut index = 0;

    for block_num in 0..n_blocks {
        // get the current block
        let block: &str = &bit_string[index..(index + block_size_m)];
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

    // now compute the chi_square statistic
    let mut observed: f64 = 0.0;

    for pi in pi_i {
        log::trace!("pi: {}", pi);
        observed += (pi - 1.0 / 2.0).powf(2.0);
    }
    log::info!("Calculated observed value {}", observed);

    let chi_square: f64 = 4.0 * (block_size_m as f64) * observed;
    log::info!("Chi square for given bit string: {}", chi_square);

    // finally, compute the p-value using the incomplete gamma function igamc
    let p_value = statrs::function::gamma::gamma_ur((n_blocks as f64) / 2.0, chi_square / 2.0);
    log::info!("p-value of bit string is {}", p_value);

    Ok(p_value >= 0.01)
}
