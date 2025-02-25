//! This module contains all of the constants used in the test suite.

/// Paths to the test files containing bit strings
pub const PI_FILE: &str = "/src/testdata/data.pi";
pub const E_FILE: &str = "/src/testdata/data.e";
pub const SQRT_2_FILE: &str = "/src/testdata/data.sqrt2";
pub const SQRT_3_FILE: &str = "/src/testdata/data.sqrt3";
pub const SHA_3_FILE: &str = "/src/testdata/data.sha3";

/// Treshold for p-value to check if bit string is random or not
pub const P_VALUE_THRESHOLD: f64 = 0.01;

/// Usual recommended size for several tests
pub const RECOMMENDED_SIZE: usize = 100;

/// Recommended size for "Discrete Fourier Transform (Spectral) Test
pub const RECOMMENDED_SIZE_DFT: usize = 1000;

/// Constants for the "Longest Run of Ones in a Block" test
pub const MIN_LENGTH: usize = 128;
pub const MID_LENGTH: usize = 6272;
pub const MAX_LENGTH: usize = 750000;

pub const MIN_SIZE_M: usize = 8;
pub const MID_SIZE_M: usize = 128;
pub const MAX_SIZE_M: usize = 10000;

pub const MIN_SIZE_N: usize = 16;
pub const MID_SIZE_N: usize = 49;
pub const MAX_SIZE_N: usize = 75;

pub const MIN_THRESHOLDS: (i32, i32) = (1, 4);
pub const MID_THRESHOLDS: (i32, i32) = (4, 9);
pub const MAX_THRESHOLDS: (i32, i32) = (10, 16);

pub static MIN_PI_VALUES: [f64; 4] = [0.21484375, 0.3671875, 0.23046875, 0.1875];
pub static MID_PI_VALUES: [f64; 6] = [
    0.1174035788,
    0.242955959,
    0.249363483,
    0.17517706,
    0.102701071,
    0.112398847,
];
pub static MAX_PI_VALUES: [f64; 7] = [0.0882, 0.2092, 0.2483, 0.1933, 0.1208, 0.0675, 0.0727];

/// Constants for the "Binary Matrix Rank" test
pub const RECOMMENDED_SIZE_MATRIX_TEST: usize = 38912;

pub const MATRIX_ROWS_M: usize = 32;
pub const MATRIX_COLUMNS_Q: usize = 32;

pub static APPROXIMATIONS: [f64; 3] = [0.2888, 0.5776, 0.1336];

/// Constants for the "Discrete Fourier Transform (Spectral)" Test
pub const LOG_ARG: f64 = 1.0 / 0.05;
pub const N_0_CONSTANT: f64 = 0.95 * 0.5;
pub const NORMALIZED_DIFF_CONSTANT: f64 = 0.95 * 0.05 * 0.25;

/// Constants for the "Non-overlapping Template Matching" Test
pub const TEMPLATE_LEN: (usize, usize) = (2, 21);
pub const RECOMMENDED_TEMPLATE_LEN: (usize, usize) = (9, 10);
pub const TEMPLATE_SUB_PATH: &str = "/templates/template";
pub const TMP_DIR: &str = "/tmp";

/// Constants for the "Overlapping Template Matching" Test
pub const RECOMMENDED_SIZE_OVERLAPPING_TEMPLATE: usize = 1000000;
pub const MAX_N_OVERLAPPING_TEMPLATE: f64 = 5.0;
pub static PI_VALUES_OVERLAPPING_TEMPLATE: [f64; 6] =
    [0.364091, 0.185659, 0.139381, 0.100571, 0.0704323, 0.139865];
