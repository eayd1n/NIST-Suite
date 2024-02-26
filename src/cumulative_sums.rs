//! This module performs the Cumulative Sums (Cusum) Test.
//!
//! Description of test from NIST SP 800-22:
//!
//! "The focus of this test is the maximal excursion (from zero) of the random walk defined by the cumulative
//! sum of adjusted (-1, +1) digits in the sequence. The purpose of the test is to determine whether the
//! cumulative sum of the partial sequences occurring in the tested sequence is too large or too small relative
//! to the expected behavior of that cumulative sum for random sequences. This cumulative sum may be
//! considered as a random walk. For a random sequence, the excursions of the random walk should be near
//! zero. For certain types of non-random sequences, the excursions of this random walk from zero will be
//! large."

use anyhow::Result;
use statrs::distribution::ContinuousCDF;

const RECOMMENDED_SIZE: usize = 100;

#[derive(Debug, PartialEq)]
pub enum Mode {
    Forward,
    Backward,
}

/// Perform the Cumulative Sums Test.
///
/// # Arguments
///
/// bit_string - The bit string to be tested for randomness
/// mode - A switch to process forward (mode = 0) or backward (mode = 1) through sequence
///
/// # Return
///
/// Ok(p-value) - The p-value which indicates whether randomness is given or not
/// Err(err) - Some error occured
pub fn perform_test(bit_string: &str, mode: Mode) -> Result<f64> {
    log::trace!("cumulative_sums::perform_test()");

    // check validity of passed bit string
    if bit_string.is_empty() || bit_string.chars().any(|c| c != '0' && c != '1') {
        anyhow::bail!("Bit string either is empty or contains invalid character(s)");
    }

    log::debug!("Chosen mode: {:?}", mode);

    let length = bit_string.len();
    log::debug!("Bit string has the length {}", length);

    // Recommended size is at least 100 bits. It is not an error but log a warning anyways
    if length < RECOMMENDED_SIZE {
        log::warn!(
            "Recommended size is at least 100 bits. Consider imprecision when calculating p-value"
        );
    }

    // Create cumulative sums depending on chosen mode
    let new_bit_string = if mode == Mode::Forward {
        bit_string.to_string()
    } else {
        bit_string.chars().rev().collect::<String>()
    };

    // now compute the particular sums and determine the maximum sum
    let mut max_sum_z: i64 = 0;
    let mut current_sum: i64 = 0;

    for (index, _) in new_bit_string.chars().enumerate() {
        for j in 0..(index + 1) {
            let window_value = if new_bit_string.chars().nth(j).unwrap() == '1' {
                1
            } else {
                -1
            };
            current_sum += window_value;
            if current_sum.abs() > max_sum_z {
                max_sum_z = current_sum.abs();
            }
        }
        // reset current sum
        current_sum = 0;
    }
    log::debug!(
        "Determined maximum value z of cumulative sums: {}",
        max_sum_z
    );

    // compute lower and upper limits before generating p-value
    let upper_limit = (((length as f64) / (max_sum_z as f64) - 1.0) * 0.25) as i64;
    let lower_limit_1 = ((-1.0 * (length as f64) / (max_sum_z as f64) + 1.0) * 0.25) as i64;
    let lower_limit_2 = ((-1.0 * (length as f64) / (max_sum_z as f64) - 3.0) * 0.25) as i64;
    log::debug!(
        "Upper limit: {}, Lower Limit 1: {}, Lower Limit 2: {}",
        upper_limit,
        lower_limit_1,
        lower_limit_2
    );

    // finally, compute p-value with the standard normal cumulative probability distribution
    // function
    let mut sum_result = 0.0;
    let normal = statrs::distribution::Normal::new(0.0, 1.0).unwrap();
    let denominator = (length as f64).sqrt();

    for k in lower_limit_1..=upper_limit {
        let numerator_1 = (4.0 * (k as f64) + 1.0) * (max_sum_z as f64);
        let numerator_2 = (4.0 * (k as f64) - 1.0) * (max_sum_z as f64);

        sum_result += normal.cdf(numerator_1 / denominator) - normal.cdf(numerator_2 / denominator);
    }

    for k in lower_limit_2..=upper_limit {
        let numerator_1 = (4.0 * (k as f64) + 3.0) * (max_sum_z as f64);
        let numerator_2 = (4.0 * (k as f64) + 1.0) * (max_sum_z as f64);

        sum_result += normal.cdf(numerator_1 / denominator) - normal.cdf(numerator_2 / denominator);
    }

    let p_value = 1.0 - sum_result;
    log::info!(
        "Cumulative Sums (Cusum): p-value of bit string in '{:?}' mode is {}",
        mode,
        p_value
    );

    Ok(p_value)
}
