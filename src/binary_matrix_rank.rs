//! This module performs the Binary Matrix Rank Test.
//!
//! Description of test from NIST SP 800-22:
//!
//! "The focus of the test is the rank of disjoint sub-matrices of the entire sequence. The purpose of this test is
//! to check for linear dependence among fixed length substrings of the original sequence."

use crate::constants;
use crate::utils;
use anyhow::Result;
use std::collections::HashMap;

/// Perform the Binary Matrix Rank Test by determining the p-value.
///
/// # Arguments
///
/// bit_string - The bit string to be tested for randomness
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

    let length = utils::evaluate_bit_string(bit_string, constants::RECOMMENDED_SIZE_MATRIX_TEST)?;

    // the test is optimized for M = Q = 32 and a bit size of n = 32 * 32 * 38. If the values are
    // not matching, log a warning because approximations do not fit anymore
    if matrix_rows_m != constants::MATRIX_ROWS_M {
        log::warn!(
            "Recommended size for rows: {}, passed rows: {}",
            constants::MATRIX_ROWS_M,
            matrix_rows_m
        );
    }
    if matrix_columns_q != constants::MATRIX_COLUMNS_Q {
        log::warn!(
            "Recommended size for columns: {}, passed columns: {}",
            constants::MATRIX_COLUMNS_Q,
            matrix_columns_q
        );
    }

    log::debug!(
        "Rows M = {}, Columns Q = {}, Length of bit string = {}",
        matrix_rows_m,
        matrix_columns_q,
        length
    );

    // create matrices from the given bit string by iterating over chunks of size M * Q
    let matrices = construct_matrices(bit_string, matrix_rows_m, matrix_columns_q);

    // determine the rank of each matrix and count their occurences
    let mut rank_counts: HashMap<usize, usize> = HashMap::new();

    for matrix in &matrices {
        // Convert the matrix to a dynamically-sized matrix before calculating the rank
        let dynamic_matrix = nalgebra::DMatrix::from_fn(matrix.nrows(), matrix.ncols(), |i, j| {
            matrix[(i, j)] as f64
        });
        *rank_counts
            .entry(dynamic_matrix.rank(1e-10) as usize)
            .or_insert(0) += 1;
    }
    log::debug!("Counts of ranks: {:?}", rank_counts);

    // determine the number of full ranks F_M, one below full ranks F_(M - 1)and the remaining
    // ranks (N - F_M - F_(M - 1)
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

    let remaining_ranks = matrices.len() - full_rank_m - full_rank_m_minus_one;
    log::debug!(
        "Number of full rank matrices: {}, full rank - 1 matrices: {}, remaining matrices: {}",
        full_rank_m,
        full_rank_m_minus_one,
        remaining_ranks
    );

    // Compute chi_square statistics
    let n_matrices = length / (matrix_rows_m * matrix_columns_q);
    let first_fraction = ((full_rank_m as f64) - constants::APPROXIMATIONS[0]).powf(2.0)
        / constants::APPROXIMATIONS[0];
    let second_fraction = ((full_rank_m_minus_one as f64) - constants::APPROXIMATIONS[1]).powf(2.0)
        / constants::APPROXIMATIONS[1];
    let third_fraction = ((remaining_ranks as f64) - constants::APPROXIMATIONS[2]).powf(2.0)
        / constants::APPROXIMATIONS[2];

    let chi_square = first_fraction + second_fraction + third_fraction;
    log::debug!("Chi_square value: {}", chi_square);

    // finally, compute p-value with exp(-chi_square * 0.5)
    let p_value = (-chi_square * 0.5).exp();
    log::info!("Binary Matrix Rank: p-value of bit string is {}", p_value);

    Ok(p_value)
}

fn construct_matrices(bit_string: &str, rows: usize, columns: usize) -> Vec<nalgebra::DMatrix<u8>> {
    log::trace!("binary_matrix_rank::construct_matrices()");

    let total_elements = rows * columns;
    let mut matrices = Vec::new();

    // Divide the bitstring into substrings of length rows * columns
    let binding = bit_string.chars().collect::<Vec<_>>();
    let substrings = binding.chunks(total_elements);

    // Iterate over the substrings to construct matrices
    for chunk in substrings {
        if chunk.len() == total_elements {
            let mut matrix = nalgebra::DMatrix::from_element(rows, columns, 0);
            for (index, &bit) in chunk.iter().enumerate() {
                let row = index / columns;
                let col = index % columns;
                matrix[(row, col)] = bit.to_digit(2).unwrap() as u8;
            }
            log::trace!("Constructed matrix: {}", &matrix);
            matrices.push(matrix);
        }
    }

    log::debug!("Number of constructed matrices: {}", &matrices.len());
    matrices
}

fn compute_fraction(
    full_rank_matrices: usize,
    full_rank_minus_one: usize,
    n_matrices: usize,
    approximation: f64,
) -> f64 {
}
