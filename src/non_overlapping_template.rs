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
use std::io::{BufRead, BufReader};

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

    // check if we got bit string only containing zeros or ones
    if bit_string.chars().all(|c| c == '0') || bit_string.chars().all(|c| c == '1') {
        anyhow::bail!("Given bit string either contains only zeros or only ones");
    }

    // evaluate the other input and get the block size m
    let block_size = evaluate_test_params(length, template_len, number_of_blocks)
        .with_context(|| "Template length does not match defined requirements")?;

    // calculate number of templates
    let number_of_templates = 2_usize.pow(template_len.try_into().unwrap()) as f64;

    // calculate theoretical mean and variance
    let first_fraction = 1.0 / number_of_templates;
    let second_fraction =
        (2.0 * (template_len as f64) - 1.0) / 2.0_f64.powf(2.0 * (template_len as f64));

    let mean = ((block_size.wrapping_sub(template_len) + 1) as f64) / number_of_templates;
    let variance = (block_size as f64) * (first_fraction - second_fraction);
    log::debug!(
        "{}: Theoretical mean = {}, Variance = {}",
        TEST_NAME,
        mean,
        variance
    );

    // now iterate over each template and search for it in each substring
    let mut p_values = Vec::<f64>::new();
    let templates = get_templates(template_len).with_context(|| "Failed to get templates")?;
    p_values.reserve_exact(templates.len());

    for template in templates {
        let mut template_counters = Vec::<usize>::new();

        // now iterate over blocks 1...N and count occurences of respective aperiodic template in substring
        for block in 0..number_of_blocks {
            let start_index = block * block_size;
            let end_index = (block + 1) * block_size;
            let substring = &bit_string[start_index..end_index];

            let mut counter = 0;
            let mut index = 0;

            while let Some(start) = substring[index..].find(&template) {
                counter += 1;

                // move the index to the next possible occurence
                index += start + template_len;
            }

            log::trace!(
                "{}: Template '{}' in substring '{}' found {} times",
                TEST_NAME,
                template,
                substring,
                counter
            );
            template_counters.push(counter);
        }
        // compute chi_square statistics
        let mut chi_square = 0.0;
        for counter in &template_counters {
            chi_square += ((*counter as f64) - mean).powf(2.0) / variance;
        }
        log::trace!(
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

        if p_value < constants::P_VALUE_THRESHOLD {
            log::warn!(
                "{}: p-value ({}) for template '{}' is below threshold",
                TEST_NAME,
                p_value,
                &template
            );
        }

        log::trace!(
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

/// Evaluate passed test parameters and return the resulting block size M.
///
/// # Arguments
///
/// bit_string_length - Length of bit string
/// template_len - Length of template to be searched later in substrings
/// number_of_blocks - The number of blocks the bitstring has to be divided into
///
/// # Return
///
/// Ok(block_size) - The resulting block size if template length is okay
/// Err(err) - Some error occured
fn evaluate_test_params(
    bit_string_length: usize,
    template_len: usize,
    number_of_blocks: usize,
) -> Result<usize> {
    log::trace!("non_overlapping_template::evaluate_test_params()");

    // check whether template length is between thresholds for meaningful results
    if !(constants::TEMPLATE_LEN.0..=constants::TEMPLATE_LEN.1).contains(&template_len) {
        anyhow::bail!(
            "{}: Passed template length '{}' must be between {} and {}",
            TEST_NAME,
            template_len,
            constants::TEMPLATE_LEN.0,
            constants::TEMPLATE_LEN.1
        );
    }

    // recommended sizes for template lengths: 9, 10. Log a warning if they do not match
    if !(constants::RECOMMENDED_TEMPLATE_LEN.0..=constants::RECOMMENDED_TEMPLATE_LEN.1)
        .contains(&template_len)
    {
        log::warn!(
            "{}: Recommended size for template length: {}, {}",
            TEST_NAME,
            constants::RECOMMENDED_TEMPLATE_LEN.0,
            constants::RECOMMENDED_TEMPLATE_LEN.1
        );
    }

    // check number of blocks
    if number_of_blocks > constants::RECOMMENDED_SIZE {
        anyhow::bail!(
            "{}: Number of blocks N ({}) is greater than recommended size ({})",
            TEST_NAME,
            number_of_blocks,
            constants::RECOMMENDED_SIZE
        );
    }

    // construct block size M to get the substrings to be tested
    let block_size = bit_string_length / number_of_blocks;
    let recommended_size = bit_string_length / 100;

    if block_size <= recommended_size {
        anyhow::bail!(
            "{}: Block size M ({}) is less than or equal to {}. Choose smaller number of blocks",
            TEST_NAME,
            block_size,
            recommended_size
        );
    }

    log::info!(
        "{}: Template length = {}, Block size M = {}, Number of blocks N = {}",
        TEST_NAME,
        template_len,
        block_size,
        number_of_blocks
    );

    Ok(block_size)
}

/// Get pre-computed templates based on passed template length.
///
/// # Arguments
///
/// template_len - Length of templates to be used for the test
///
/// # Return
///
/// Ok(templates) - The extracted templates from file
/// Err(err) - Some error occured
fn get_templates(template_len: usize) -> Result<Vec<String>> {
    log::trace!("non_overlapping_template::get_templates()");

    // check whether template file already exists in /tmp (due to previous runs). Therefore no
    // unpacking needed anymore
    let template_file_path =
        constants::TMP_DIR.to_owned() + "/template" + &template_len.to_string();
    if !std::path::Path::new(&template_file_path).exists() {
        // create path to templates to use
        let template_path = match std::env::current_dir() {
            Ok(path) => path,
            Err(err) => anyhow::bail!("Failed to retrieve current working directory: {}", err),
        };

        let template_archive = template_path.to_string_lossy().into_owned()
            + constants::TEMPLATE_SUB_PATH
            + &template_len.to_string()
            + ".tar.gz";

        // now unpack the archive to tmp directory and read in the particular templates from file
        utils::untar_archive(&template_archive, constants::TMP_DIR).with_context(|| {
            format!("Failed to unpack template archive '{}'", &template_archive)
        })?;
    }

    // read the file contents line by line
    let template_file = std::fs::File::open(&template_file_path)
        .with_context(|| format!("Failed to open template file '{}'", &template_file_path))?;

    let reader = BufReader::new(template_file);
    let mut templates = Vec::<String>::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            templates.push(line);
        }
    }

    log::info!("Extracted {} templates to test with", templates.len());

    Ok(templates)
}
