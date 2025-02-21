//! This module performs the Binary Matrix Rank Test.
//!
//! Description of test from NIST SP 800-22:
//!
//! "The focus of the test is the rank of disjoint sub-matrices of the entire sequence. The purpose of this test is
//! to check for linear dependence among fixed length substrings of the original sequence."

use crate::constants;
use crate::customtypes;
use crate::utils;
use anyhow::Result;
use std::collections::HashMap;

const TEST_NAME: customtypes::Test = customtypes::Test::BinaryMatrixRank;

/// Perform the Binary Matrix Rank Test by determining the p-value.
///
/// # Arguments
///
/// bit_string - The bit string to be tested for randomnesshas been running for over 60 seconds
/// matrix_rows_m - The numbers of rows all matrices need to have
/// matrix_columns_q - The number of columns all matrices need to have
///
/// # Return
///
/// Ok(p-value) - The p-value which indicates whether randomness is given or not
/// Err(err) - Some error occured
pub fn perform_test(
    bit_string: &str,
    matrix_rows_m: usize,
    matrix_columns_q: usize,
) -> Result<f64> {
    log::trace!("binary_matrix_rank::perform_test()");

    // capture the current time before executing the actual test
    let start_time = std::time::Instant::now();

    // check if bit string contains invalid characters
    let length = utils::evaluate_bit_string(
        TEST_NAME,
        bit_string,
        constants::RECOMMENDED_SIZE_MATRIX_TEST,
    )?;

    // the test is optimized for M = Q = 32 and a bit size of n = 32 * 32 * 38. If the values are
    // not matching, log a warning because approximations may not fit anymore
    if matrix_rows_m != constants::MATRIX_ROWS_M {
        log::warn!(
            "{}: Recommended size for rows: {}, passed rows: {}",
            TEST_NAME,
            constants::MATRIX_ROWS_M,
            matrix_rows_m
        );
    }
    if matrix_columns_q != constants::MATRIX_COLUMNS_Q {
        log::warn!(
            "{}: Recommended size for columns: {}, passed columns: {}",
            TEST_NAME,
            constants::MATRIX_COLUMNS_Q,
            matrix_columns_q
        );
    }

    // create matrices from the given bit string by iterating over chunks of size M * Q
    let matrices = construct_matrices(bit_string, matrix_rows_m, matrix_columns_q);

    // determine the rank of each matrix and count their occurences
    let n_matrices = length / (matrix_rows_m * matrix_columns_q);
    let mut rank_counts: HashMap<usize, usize> = HashMap::new();

    for mut matrix in matrices.into_iter() {
        *rank_counts.entry(compute_rank(&mut matrix)).or_insert(0) += 1;
    }

    log::debug!("{}: Counts of ranks: {:?}", TEST_NAME, rank_counts);

    // determine the number of full ranks F_M, full ranks F_(M - 1) and the remaining
    // ranks (N - F_M - F_(M - 1))
    let full_rank_m = if let Some(&full_rank) = rank_counts.get(&matrix_rows_m) {
        full_rank
    } else {
        0
    };

    let full_rank_m_minus_one =
        if let Some(&full_rank_below) = rank_counts.get(&(matrix_rows_m - 1)) {
            full_rank_below
        } else {
            0
        };

    let remaining_ranks = n_matrices - full_rank_m - full_rank_m_minus_one;

    log::debug!(
        "{}: Number of full rank matrices: {}, full rank - 1 matrices: {}, remaining matrices: {}",
        TEST_NAME,
        full_rank_m,
        full_rank_m_minus_one,
        remaining_ranks
    );

    // Compute chi_square statistics by calculating the three fractions (one fraction per rank)
    let first_fraction = compute_fraction(full_rank_m, n_matrices, constants::APPROXIMATIONS[0]);
    let second_fraction = compute_fraction(
        full_rank_m_minus_one,
        n_matrices,
        constants::APPROXIMATIONS[1],
    );
    let third_fraction =
        compute_fraction(remaining_ranks, n_matrices, constants::APPROXIMATIONS[2]);

    let chi_square = first_fraction + second_fraction + third_fraction;
    log::debug!("{}: Chi_square value: {}", TEST_NAME, chi_square);

    // finally, compute p-value with exp(-chi_square / 2.0)
    let p_value = (-chi_square * 0.5).exp();
    log::info!("{}: p-value = {}", TEST_NAME, p_value);

    // capture the current time after the test got executed and calculate elapsed time
    let end_time = std::time::Instant::now();
    let elapsed_time = end_time.duration_since(start_time).as_secs_f64();
    log::info!("{} took {:.6} seconds", TEST_NAME, elapsed_time);

    Ok(p_value)
}

/// Construct matrices from the given bit string.
///
/// # Arguments
///
/// bit_string - The bit string the matrices have to be constructed from
/// rows - The number of rows the matrices will have
/// columns - The number of columns the matrices will have
///
/// # Return
///
/// matrices - All of the constructed matrices
fn construct_matrices(
    bit_string: &str,
    rows: usize,
    columns: usize,
) -> Vec<nalgebra::DMatrix<rug::Integer>> {
    log::trace!("binary_matrix_rank::construct_matrices()");

    let total_elements = rows * columns;
    let mut matrices = Vec::new();

    // Divide the bitstring into substrings of length rows * columns
    let binding = bit_string.chars().collect::<Vec<_>>();
    let substrings = binding.chunks(total_elements);
    log::debug!(
        "{}: Discarded {} bits from input",
        TEST_NAME,
        bit_string.len() % total_elements
    );

    // Iterate over the substrings to construct matrices
    for chunk in substrings {
        if chunk.len() == total_elements {
            let mut matrix = nalgebra::DMatrix::from_element(rows, columns, rug::Integer::new());
            for (index, &bit) in chunk.iter().enumerate() {
                let row = index / columns;
                let col = index % columns;
                matrix[(row, col)] = rug::Integer::from(bit.to_digit(2).unwrap());
            }
            log::trace!("{}: Constructed matrix: {}", TEST_NAME, &matrix);
            matrices.push(matrix);
        }
    }

    log::debug!(
        "{}: Number of constructed matrices: {}",
        TEST_NAME,
        &matrices.len()
    );
    matrices
}

/// Compute the rank of the given matrix.
///
/// # Arguments
///
/// matrix - The matrix the rank has to be determined from
///
/// # Return
///
/// rank - The rank of the given matrix
fn compute_rank(matrix: &mut nalgebra::DMatrix<rug::Integer>) -> usize {
    log::trace!("binary_matrix_rank::compute_rank()");

    let (mut row, mut col) = (0, 0);
    let mut rank = 0;

    while row < matrix.nrows() && col < matrix.ncols() {
        // Find the pivot for this column
        let mut max_row = row;
        for i in row + 1..matrix.nrows() {
            let abs_value_i = matrix[(i, col)].clone().abs();
            let abs_value_max_row = matrix[(max_row, col)].clone().abs();
            if abs_value_i > abs_value_max_row {
                max_row = i;
            }
        }

        if matrix[(max_row, col)].is_zero() {
            // All elements in this column are zero
            col += 1;
        } else {
            // Swap the rows to move the pivot to the current row
            matrix.swap_rows(row, max_row);

            // Perform row operations to eliminate elements below the pivot
            for i in row + 1..matrix.nrows() {
                let factor = matrix[(i, col)].clone() / matrix[(row, col)].clone();
                for j in col..matrix.ncols() {
                    let temp = factor.clone() * matrix[(row, j)].clone();
                    matrix[(i, j)] -= temp;
                }
            }

            row += 1;
            col += 1;
            rank += 1;
        }
    }

    rank
}

/// Compute the fractions needed to determine the chi_square value.
///
/// # Arguments
///
/// rank - The rank of a matrix
/// n_matrices - The overall number of matrices
/// approximation - The pre-calculated approximation for the fraction
///
/// # Return
///
/// fraction - The calculated fraction
fn compute_fraction(rank: usize, n_matrices: usize, approximation: f64) -> f64 {
    log::trace!("binary_matrix::rank::compute_fraction()");

    let constant = approximation * (n_matrices as f64);
    let fraction = ((rank as f64) - constant).powf(2.0) / constant;

    log::debug!("{}: Computed fraction: {}", TEST_NAME, fraction);
    fraction
}

#[cfg(test)]
mod tests {
    use crate::binary_matrix_rank;
    use crate::constants;
    use crate::logger;
    use crate::utils;

    const LOGLEVEL: &str = "Debug";
    const BIT_STRING_1: &str = "01011001001010101101"; // example from NIST Paper. p-value should be 0.741948
    const INVALID_BIT_STRING: &str = "010101111010101010101010101010a0101010101010100101010101";
    const PI_FILE: &str = "/src/testdata/data.pi";
    const E_FILE: &str = "/src/testdata/data.e";
    const SQRT_2_FILE: &str = "/src/estdata/data.sqrt2";
    const SQRT_3_FILE: &str = "/src/testdata/data.sqrt3";
    const SHA_3_FILE: &str = "/src/testdata/data.sha3";

    // XXX Fix binary matrix rank
    #[test]
    fn test_binary_matrix_rank() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        assert!(binary_matrix_rank::perform_test(BIT_STRING_1, 3, 3).unwrap() >= 0.01);

        // test pi, e, sqrt(2) and sqrt(3) in their respective binary representations
        let pi_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + PI_FILE;
        let pi_bit_string = utils::read_random_numbers(&pi_file).unwrap();
        assert!(
            binary_matrix_rank::perform_test(
                &pi_bit_string,
                constants::MATRIX_ROWS_M,
                constants::MATRIX_COLUMNS_Q
            )
            .unwrap()
                >= 0.01
        );

        let e_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + E_FILE;
        let e_bit_string = utils::read_random_numbers(&e_file).unwrap();
        assert!(
            binary_matrix_rank::perform_test(
                &e_bit_string,
                constants::MATRIX_ROWS_M,
                constants::MATRIX_COLUMNS_Q
            )
            .unwrap()
                >= 0.01
        );

        let sqrt_2_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + SQRT_2_FILE;
        let sqrt_2_bit_string = utils::read_random_numbers(&sqrt_2_file).unwrap();
        assert!(
            binary_matrix_rank::perform_test(
                &sqrt_2_bit_string,
                constants::MATRIX_ROWS_M,
                constants::MATRIX_COLUMNS_Q
            )
            .unwrap()
                >= 0.01
        );

        let sqrt_3_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + SQRT_3_FILE;
        let sqrt_3_bit_string = utils::read_random_numbers(&sqrt_3_file).unwrap();
        assert!(
            binary_matrix_rank::perform_test(
                &sqrt_3_bit_string,
                constants::MATRIX_ROWS_M,
                constants::MATRIX_COLUMNS_Q
            )
            .unwrap()
                >= 0.01
        );

        let sha_3_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + SHA_3_FILE;
        let sha_3_bit_string = utils::read_random_numbers(&sha_3_file).unwrap();
        assert!(
            binary_matrix_rank::perform_test(
                &sha_3_bit_string,
                constants::MATRIX_ROWS_M,
                constants::MATRIX_COLUMNS_Q
            )
            .unwrap()
                >= 0.01
        );
    }

    #[test]
    fn test_binary_matrix_rank_error_cases() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        // pass empty string
        assert!(binary_matrix_rank::perform_test("", 3, 3).is_err());

        // pass invalid bit string
        assert!(binary_matrix_rank::perform_test(INVALID_BIT_STRING, 32, 32).is_err());
    }
}
