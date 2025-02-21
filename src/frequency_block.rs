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
use anyhow::{bail, Context, Result};

const TEST_NAME: customtypes::Test = customtypes::Test::FrequencyBlock;

/// Perform the "Frequency within a block" test.
///
/// # Arguments
///
/// bit_string - The bit string to be tested for randomness
/// block_size - Divide the bit string into equal blocks of size M
///
/// # Return
///
/// Ok(p-value) - The p-value which indicates whether randomness is given or not
/// Err(err) - Some error occured
pub fn perform_test(bit_string: &str, block_size: usize) -> Result<f64> {
    log::trace!("frequency_block::perform_test()");

    // capture the current time before executing the actual test
    let start_time = std::time::Instant::now();

    // check if bit string contains invalid characters
    let length = utils::evaluate_bit_string(TEST_NAME, bit_string, constants::RECOMMENDED_SIZE)
        .with_context(|| "Invalid character(s) in passed bit string detected")?;

    // check block size M for validity and get number of blocks N
    let number_of_blocks = evaluate_block_size(length, block_size).with_context(|| {
        format!(
            "{TEST_NAME}: Either block size M or number of blocks N does not fit to defined requirements")
    })?;

    // Calculate pi_i = #ones_per_block/block_size
    let pi_i = compute_pi_i(bit_string, number_of_blocks, block_size);

    // now compute the chi_square statistics: chi_square = 4 * M * sum(p_i - 0.5)^2
    let chi_square = compute_chi_square(block_size, pi_i);

    // finally, compute the p-value using the incomplete gamma function: igamc(N/2, chi_square/2)
    // Note: If we do have a perfect distribution (M/2 ones in each block), chi_square is zero
    // which is an invalid input for igamc. Return p-value of 1 then
    let p_value = if chi_square == 0.0 {
        1.0
    } else {
        statrs::function::gamma::gamma_ur((number_of_blocks as f64) * 0.5, chi_square * 0.5)
    };
    log::info!("{TEST_NAME}: p-value = {p_value}");

    // capture the current time after the test got executed and calculate elapsed time
    let end_time = std::time::Instant::now();
    let elapsed_time = end_time.duration_since(start_time).as_secs_f64();
    log::info!("{TEST_NAME} took {:.6} seconds", elapsed_time);

    Ok(p_value)
}

fn evaluate_block_size(length: usize, block_size: usize) -> Result<usize> {
    log::trace!("frequency_block::evaluate_block_size()");

    // M should be less than bit string length but greater than (length / 100)
    if block_size >= length || block_size <= (length / constants::RECOMMENDED_SIZE) {
        bail!(
            "{TEST_NAME}: Choose block size as of {} < M < {}",
            length / constants::RECOMMENDED_SIZE,
            length
        );
    }

    // calculate number of blocks N by floor(length/block_size). N should be < 100
    let number_of_blocks = length / block_size;
    if number_of_blocks >= constants::RECOMMENDED_SIZE {
        bail!(
            "{TEST_NAME}: Number of blocks exceed {}: {}. Please choose a larger M",
            constants::RECOMMENDED_SIZE,
            number_of_blocks
        );
    }

    log::info!(
        "{TEST_NAME}: Block size M: {}, number of blocks N to proceed: {}",
        block_size,
        number_of_blocks
    );

    Ok(number_of_blocks)
}

fn compute_pi_i(bit_string: &str, number_of_blocks: usize, block_size: usize) -> Vec<f64> {
    log::trace!("frequncy_block::compute_pi_i()");

    let mut pi_i = Vec::<f64>::new();
    pi_i.reserve_exact(number_of_blocks);

    let mut index = 0;

    for current_block in 0..number_of_blocks {
        let block = &bit_string[index..(index + block_size)];
        let count_ones = block.chars().filter(|&c| c == '1').count() as f64;
        log::trace!(
            "{TEST_NAME}: Block {}/{}: '{}' consists of {} ones",
            current_block + 1,
            number_of_blocks,
            block,
            count_ones
        );

        pi_i.push(count_ones / (block_size as f64));

        index += block_size;
    }

    pi_i
}

fn compute_chi_square(block_size: usize, pi_i: Vec<f64>) -> f64 {
    log::trace!("frequency_block::compute_chi_square()");

    let mut observed = 0.0;

    for (index, pi) in pi_i.iter().enumerate() {
        log::trace!("pi_{}: {}", index + 1, pi);
        observed += (pi - 0.5).powf(2.0);
    }
    log::debug!("{TEST_NAME}: Calculated observed value {observed}");

    let chi_square = 4.0 * (block_size as f64) * observed;
    log::debug!("{TEST_NAME}: Chi square for given bit string: {chi_square}");

    chi_square
}

#[cfg(test)]
mod tests {
    use crate::constants;
    use crate::frequency_block;
    use crate::logger;
    use crate::utils;

    const LOGLEVEL: &str = "Info";
    const BIT_STRING_NIST_1: &str = "0110011010";
    const P_VALUE_NIST_1: f64 = 0.8012519569012013;
    const BIT_STRING_NIST_2: &str = "1100100100001111110110101010001000100001011010001100001000110100110001001100011001100010100010111000";
    const P_VALUE_NIST_2: f64 = 0.7064384496412821;
    const BIT_STRING_ONLY_ZEROS: &str = "0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
    const BIT_STRING_ONLY_ONES: &str = "1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111";
    const BIT_STRING_RANDOM: &str = "11101000100101110100010110100101111100000101010101000101110101010101011101101010010101000001011101110101";
    const BIT_STRING_NON_RANDOM: &str = "0000000000100000000000000000000000000100000000000000000000000000000010000000000000000000000000001000";
    const BIT_STRING_PERFECT: &str = "1111100000111110000011111000001111100000111110000011111000001111100000111110000011111000001111100000";
    const INVALID_BIT_STRING: &str = "010101111010101010101010101010a0101010101010100101010101";

    #[test]
    fn test_frequency_block() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        assert_eq!(
            frequency_block::perform_test(BIT_STRING_NIST_1, 3).unwrap(),
            P_VALUE_NIST_1
        );
        assert_eq!(
            frequency_block::perform_test(BIT_STRING_NIST_2, 10).unwrap(),
            P_VALUE_NIST_2
        );
        assert!(frequency_block::perform_test(BIT_STRING_ONLY_ZEROS, 10).unwrap() < 0.01);
        assert!(frequency_block::perform_test(BIT_STRING_ONLY_ONES, 10).unwrap() < 0.01);
        assert_eq!(
            frequency_block::perform_test(BIT_STRING_ONLY_ZEROS, 10).unwrap(),
            frequency_block::perform_test(BIT_STRING_ONLY_ONES, 10).unwrap()
        );
        assert!(frequency_block::perform_test(BIT_STRING_RANDOM, 10).unwrap() >= 0.01);
        assert!(frequency_block::perform_test(BIT_STRING_NON_RANDOM, 20).unwrap() < 0.01);
        assert!(frequency_block::perform_test(BIT_STRING_PERFECT, 10).unwrap() == 1.00);

        // test pi, e, sqrt(2) and sqrt(3) in their respective binary representations
        let pi_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + constants::PI_FILE;
        let pi_bit_string = utils::read_random_numbers(&pi_file).unwrap();
        assert!(frequency_block::perform_test(&pi_bit_string, 10200).unwrap() >= 0.01);

        let e_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + constants::E_FILE;
        let e_bit_string = utils::read_random_numbers(&e_file).unwrap();
        assert!(frequency_block::perform_test(&e_bit_string, 10200).unwrap() >= 0.01);

        let sqrt_2_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + constants::SQRT_2_FILE;
        let sqrt_2_bit_string = utils::read_random_numbers(&sqrt_2_file).unwrap();
        assert!(frequency_block::perform_test(&sqrt_2_bit_string, 10200).unwrap() >= 0.01);

        let sqrt_3_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + constants::SQRT_3_FILE;
        let sqrt_3_bit_string = utils::read_random_numbers(&sqrt_3_file).unwrap();
        assert!(frequency_block::perform_test(&sqrt_3_bit_string, 10200).unwrap() >= 0.01);

        let sha_3_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + constants::SHA_3_FILE;
        let sha_3_bit_string = utils::read_random_numbers(&sha_3_file).unwrap();
        assert!(frequency_block::perform_test(&sha_3_bit_string, 10250).unwrap() >= 0.01);
    }

    #[test]
    fn test_frequency_block_error_cases() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        // pass empty string
        assert!(frequency_block::perform_test("", 10).is_err());

        // pass invalid bit string
        assert!(frequency_block::perform_test(INVALID_BIT_STRING, 10).is_err());

        // pass wrong sizes of M
        assert!(
            frequency_block::perform_test(BIT_STRING_NON_RANDOM, BIT_STRING_NON_RANDOM.len())
                .is_err()
        );
        assert!(frequency_block::perform_test(BIT_STRING_NON_RANDOM, 0).is_err());
    }
}
