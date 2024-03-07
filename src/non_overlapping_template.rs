//! This module performs the Non-overlapping Template Matching Test.
//!
//! Description of test from NIST SP 800-22:
//!
//! "The focus of this test is the number of occurrences of pre-specified target strings. The purpose of this
//! test is to detect generators that produce too many occurrences of a given non-periodic (aperiodic) pattern.
//! For this test and for the Overlapping Template Matching test of Section 2.8, an m-bit window is used to
//! search for a specific m-bit pattern. If the pattern is not found, the window slides one bit position. If the
//! pattern is found, the window is reset to the bit after the found pattern, and the search resumes."

use crate::constants;
use crate::customtypes;
use crate::utils;
use anyhow::{Context, Result};

const TEST_NAME: customtypes::Test = customtypes::Test::NonOverlappingTemplate;

/// Perform the Non-overlapping Template Matching Test by determining the p-value.
///
/// # Arguments
///
/// bit_string - The bit string to be tested for randomness
/// template_len - Length of templates to be used for test
/// number_of_blocks - The number of blocks the bit string has to be divided into
///
/// # Return
///
/// Ok(p-value) - The p-value which indicates whether randomness is given or not
/// Err(err) - Some error occured
pub fn perform_test(bit_string: &str, template_len: usize, number_of_blocks: usize) -> Result<f64> {
    log::trace!("non_overlapping_template::perform_test()");

    // capture the current time before executing the actual test
    let start_time = std::time::Instant::now();

    // check if bit string contains invalid characters
    let length = utils::evaluate_bit_string(TEST_NAME, bit_string, constants::RECOMMENDED_SIZE)
        .with_context(|| "Invalid character(s) in passed bit string detected")?;

    // evalute the template length
    if !(constants::TEMPLATE_LEN.0..constants::TEMPLATE_LEN.1).contains(&template_len) {
        anyhow::bail!(
            "{}: Passed template length '{}' must be between {} and {}",
            TEST_NAME,
            template_len,
            constants::TEMPLATE_LEN.0,
            constants::TEMPLATE_LEN.1
        );
    }

    // recommended sizes for template lengths: 9, 10. Log a warning if they do not match
    if template_len < constants::TEMPLATE_LEN.1 - 1 {
        log::warn!(
            "{}: Recommended size for template length: {}, {}",
            TEST_NAME,
            constants::TEMPLATE_LEN.1 - 1,
            constants::TEMPLATE_LEN.1
        );
    }

    // construct block size M to get the substrings to be tested
    let block_size_m = length / number_of_blocks;
    log::info!(
        "{}: Template length = {}, Block size M = {}, Number of blocks N = {}",
        TEST_NAME,
        template_len,
        block_size_m,
        number_of_blocks
    );

    // calculate number of templates to be searched
    let number_of_templates = 2_usize.pow(template_len.try_into().unwrap());

    // calculate theoretical mean and variance
    let first_fraction = 1.0 / (number_of_templates as f64);
    let second_fraction =
        (2.0 * (template_len as f64) - 1.0) / 2.0_f64.powf(2.0 * (template_len as f64));

    let mean = ((block_size_m - template_len + 1) as f64) / (number_of_templates as f64);
    let variance = (block_size_m as f64) * (first_fraction - second_fraction);
    log::debug!(
        "{}: Theoretical mean = {}, Variance = {}",
        TEST_NAME,
        mean,
        variance
    );

    // now iterate over each template and search for it in each substring
    let mut p_values = Vec::<f64>::new();

    for num in 0..number_of_templates {
        let template = format!("{:0width$b}", num, width = template_len as usize);
        let mut template_counters = Vec::<usize>::new();

        // now iterate over blocks 1...N and count occurences of respective template in substring
        for block in 0..number_of_blocks {
            let start_index = block * block_size_m;
            let end_index = (block + 1) * block_size_m;
            let substring = &bit_string[start_index..end_index];

            let mut current_counter = 0;
            let mut index = 0;

            while (index + template_len) < block_size_m {
                log::trace!(
                    "Index: {}, Index + Template length: {}",
                    index,
                    index + template_len
                );
                if &substring[index..(index + template_len)] == template {
                    current_counter += 1;
                    index += template_len;
                } else {
                    index += 1;
                }
            }
            log::trace!(
                "{}: Template '{}' in substring '{}' found {} times",
                TEST_NAME,
                template,
                substring,
                current_counter
            );
            template_counters.push(current_counter);
        }
        // compute chi_square statistics
        let mut chi_square = 0.0;
        for counter in &template_counters {
            chi_square += ((*counter as f64) - mean).powf(2.0) / variance;
        }
        log::debug!(
            "{}: Chi_square = {} for template '{}'",
            TEST_NAME,
            chi_square,
            template
        );

        // now compute p-value for current template with incomplete gamma function
        let p_value = if chi_square == 0.0 {
            1.0
        } else {
            statrs::function::gamma::gamma_ur((number_of_blocks as f64) * 0.5, chi_square * 0.5)
        };
        log::debug!(
            "{}: p-value = {} for template '{}'",
            TEST_NAME,
            p_value,
            template
        );

        p_values.push(p_value);
    }

    let p_values_mean = p_values.iter().sum::<f64>() / (p_values.len() as f64);
    log::info!("{}: Mean of p-values = {}", TEST_NAME, p_values_mean);

    // capture the current time after the test got executed and calculate elapsed time
    let end_time = std::time::Instant::now();
    let elapsed_time = end_time.duration_since(start_time).as_secs_f64();
    log::info!("{} took {:.6} seconds", TEST_NAME, elapsed_time);

    Ok(p_values_mean)
}
