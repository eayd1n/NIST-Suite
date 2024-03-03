//! This module contains custom types like enums and structs and their respective implementations.

/// Struct for "Longest Run of Ones in a Block" test
#[derive(Debug)]
pub struct LongestRunConfig<'a> {
    pub block_size_m: usize,
    pub n_blocks: usize,
    pub thresholds: (i32, i32),
    pub pi_values: &'a [f64],
}

impl<'a> LongestRunConfig<'a> {
    pub fn create(
        block_size_m: usize,
        n_blocks: usize,
        thresholds: (i32, i32),
        pi_values: &'a [f64],
    ) -> Self {
        LongestRunConfig {
            block_size_m,
            n_blocks,
            thresholds,
            pi_values,
        }
    }
}

/// Enum for "Cumulative Sums (Cusum)" test
#[derive(Debug, PartialEq)]
pub enum Mode {
    Forward,
    Backward,
}
