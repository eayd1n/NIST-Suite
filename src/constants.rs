//! This module contains all of the constants used in the test suite.

/// Usual recommended size for several tests
pub const RECOMMENDED_SIZE: usize = 100;

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

pub static MIN_PI_VALUES: [f64; 4] = [0.2148, 0.3672, 0.2305, 0.1875];
pub static MID_PI_VALUES: [f64; 6] = [0.1174, 0.2430, 0.2493, 0.1752, 0.1027, 0.1124];
pub static MAX_PI_VALUES: [f64; 7] = [0.0882, 0.2092, 0.2483, 0.1933, 0.1208, 0.0675, 0.0727];

/// Constants for the "Binary Matrix Rank" test
pub const RECOMMENDED_SIZE_MATRIX_TEST: usize = 38912;

pub const MATRIX_ROWS_M: usize = 32;
pub const MATRIX_COLUMNS_Q: usize = 32;

pub static APPROXIMATIONS: [f64; 3] = [0.2888, 0.5776, 0.1336];
