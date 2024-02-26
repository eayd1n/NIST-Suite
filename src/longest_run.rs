//! This module performs the Longest Run of Ones in a Block test. For this test, it is crucial to pass
//! least 128 bit!
//!
//! Description of test from NIST SP 800-22:
//!
//! "The focus of the test is the longest run of ones within M-bit blocks. The purpose of this test is to
//! determine whether the length of the longest run of ones within the tested sequence is consistent with the
//! length of the longest run of ones that would be expected in a random sequence. Note that an irregularity in
//! the expected length of the longest run of ones implies that there is also an irregularity in the expected
//! length of the longest run of zeroes. Therefore, only a test for ones is necessary."

use anyhow::Result;
use std::collections::HashMap;

const MIN_LENGTH: usize = 128;
const MID_LENGTH: usize = 6272;
const MAX_LENGTH: usize = 750000;

const MIN_SIZE_M: usize = 8;
const MID_SIZE_M: usize = 128;
const MAX_SIZE_M: usize = 10000;

const MIN_SIZE_N: usize = 16;
const MID_SIZE_N: usize = 49;
const MAX_SIZE_N: usize = 75;

const MIN_THRESHOLDS: (i32, i32) = (1, 4);
const MID_THRESHOLDS: (i32, i32) = (4, 9);
const MAX_THRESHOLDS: (i32, i32) = (10, 16);

static MIN_PI_VALUES: [f64; 4] = [0.2148, 0.3672, 0.2305, 0.1875];
static MID_PI_VALUES: [f64; 6] = [0.1174, 0.2430, 0.2493, 0.1752, 0.1027, 0.1124];
static MAX_PI_VALUES: [f64; 7] = [0.0882, 0.2092, 0.2483, 0.1933, 0.1208, 0.0675, 0.0727];

/// Perform the Longest Run of Ones in a Block test.
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
    log::trace!("longest_run::perform_test()");

    // it is crucial to have at least 128 bits passed
    let length = bit_string.len();
    if length < MIN_LENGTH {
        anyhow::bail!(
            "Bit string needs at least 128 bits! Actual length: {}",
            length
        );
    }

    // check validity of passed bit string
    if bit_string.chars().any(|c| c != '0' && c != '1') {
        anyhow::bail!("Bit string contains invalid character(s)");
    }

    log::debug!("Bit string has the length {}", length);

    // depending on length of bit string, choose the correct value for M (number of bits per block)
    // and N (number of blocks) and thresholds (min, max)
    let block_size_m;
    let n_blocks;
    let thresholds;
    let pi_values: &[f64];

    if length >= MIN_LENGTH && length < MID_LENGTH {
        block_size_m = MIN_SIZE_M;
        n_blocks = MIN_SIZE_N;
        thresholds = MIN_THRESHOLDS;
        pi_values = &MIN_PI_VALUES;
    } else if length >= MID_LENGTH && length < MAX_LENGTH {
        block_size_m = MID_SIZE_M;
        n_blocks = MID_SIZE_N;
        thresholds = MID_THRESHOLDS;
        pi_values = &MID_PI_VALUES;
    } else {
        block_size_m = MAX_SIZE_M;
        n_blocks = MAX_SIZE_N;
        thresholds = MAX_THRESHOLDS;
        pi_values = &MAX_PI_VALUES;
    }
    log::info!(
        "Block size M: {}, Number of Blocks N: {}, Thresholds to use: {:?}",
        block_size_m,
        n_blocks,
        thresholds
    );

    // determine the number of runs per block and calculate v_i
    let mut counts: HashMap<i32, i32> = HashMap::new();

    for i in (0..length).step_by(block_size_m) {
        let end_index = (i + block_size_m).min(length);
        let block = &bit_string[i..end_index];
        let max_consecutive_ones = count_max_consecutive_ones(block);

        *counts.entry(max_consecutive_ones).or_insert(0) += 1;
    }

    let vi_counts_unsorted = calculate_vi_values(counts, thresholds);
    let mut vi_counts: Vec<_> = vi_counts_unsorted.keys().cloned().collect();
    vi_counts.sort();

    // Now we need to compute chi_square value
    let mut chi_square = 0.0;

    for (key, &pi_value) in vi_counts.iter().zip(pi_values.iter()) {
        if let Some(vi_value) = vi_counts_unsorted.get(key) {
            log::trace!(
                "Current vi_value: {}, current pi_value: {}",
                *vi_value,
                pi_value
            );

            let constant = (n_blocks as f64) * pi_value;
            chi_square += ((*vi_value as f64) - constant).powf(2.0) / constant;
        }
    }
    log::debug!("Value of chi_square: {}", chi_square);

    // finally compute p-value with the incomplete gamma function
    let p_value =
        statrs::function::gamma::gamma_ur(((pi_values.len() as f64) - 1.0) / 2.0, chi_square / 2.0);
    log::debug!(
        "Longest Run Within a Block: p-value of bit string is {}",
        p_value
    );

    Ok(p_value)
}

/// Calculcate the v_i values. Those are basically counters wh<t longest run number occured how
/// often.
///
/// # Arguments
///
/// run_counts - A hashmap with collected longest run counts
/// thresholds - Minimum and maximum thresholds to merge specific counts
///
/// # Return
///
/// vi_counts - The collected v_i values
fn calculate_vi_values(run_counts: HashMap<i32, i32>, thresholds: (i32, i32)) -> HashMap<i32, i32> {
    log::trace!("longest_run::calculate_vi_values()");

    let mut vi_counts: HashMap<i32, i32> = HashMap::new();

    for (&key, &value) in &run_counts {
        if key <= thresholds.0 {
            *vi_counts.entry(thresholds.0).or_insert(0) += value;
        } else if key >= thresholds.1 {
            *vi_counts.entry(thresholds.1).or_insert(0) += value;
        } else {
            *vi_counts.entry(key).or_insert(0) += value;
        }
    }

    // If there were no counts less than or equal to the min threshold,
    // insert a zero count for the min threshold
    if !run_counts.contains_key(&thresholds.0) {
        vi_counts.insert(thresholds.0, 0);
    }

    // If there were no counts greater than or equal to the max threshold,
    // insert a zero count for the max threshold
    if !run_counts.contains_key(&thresholds.1) {
        vi_counts.insert(thresholds.1, 0);
    }

    // Print hashmap contents
    for (key, value) in &vi_counts {
        log::debug!("Calculated v_{} value: {}", key - 1, value);
    }

    vi_counts
}

/// Get the longest run of ones in a given block.
///
/// # Arguments
///
/// block - The block the longest run has to be computed from
///
/// # Return
///
/// max_count - Longest run number
fn count_max_consecutive_ones(block: &str) -> i32 {
    log::trace!("longest_run::count_max_consecutive_ones()");

    let mut max_count = 0;
    let mut current_count = 0;

    for bit in block.chars() {
        if bit == '1' {
            current_count += 1;
            max_count = max_count.max(current_count);
        } else {
            current_count = 0
        }
    }

    log::debug!("Block '{}', longest run of ones: {}", block, max_count);
    max_count
}
