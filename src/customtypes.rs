//! This module contains custom types like enums and structs and their respective implementations.

/// The names of the particular tests
pub enum Test {
    FrequencyMonobit,
    FrequencyBlock,
    Runs,
    LongestRun,
    BinaryMatrixRank,
    DFTSpectral,
    NonOverlappingTemplate,
    OverlappingTemplate,
    MaurersUniversalStatistical,
    LinearComplexity,
    Serial,
    ApproximateEntropy,
    CumulativeSums,
    RandomExcursions,
    RandomExcursionsVariant,
}

impl std::fmt::Display for Test {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Test::FrequencyMonobit => write!(f, "'Frequency Monobit Test'"),
            Test::FrequencyBlock => write!(f, "'Frequency Test within a Block'"),
            Test::Runs => write!(f, "'Runs Test'"),
            Test::LongestRun => write!(f, "'Longest Run of Ones in a Block Test'"),
            Test::BinaryMatrixRank => write!(f, "'Binary Matrix Rank Test'"),
            Test::DFTSpectral => write!(f, "'Discrete Fourier Transform (Spectral) Test'"),
            Test::NonOverlappingTemplate => write!(f, "'Non-overlapping Template Matching Test'"),
            Test::OverlappingTemplate => write!(f, "'Overlapping Template Matching Test'"),
            Test::MaurersUniversalStatistical => {
                write!(f, "'Maurer's Universal Statistical Test'")
            }
            Test::LinearComplexity => write!(f, "'Linear Complexity Test'"),
            Test::Serial => write!(f, "'Serial Test'"),
            Test::ApproximateEntropy => write!(f, "'Approximate Entropy Test'"),
            Test::CumulativeSums => write!(f, "'Cumulative Sums (Cusums) Test'"),
            Test::RandomExcursions => write!(f, "'Random Excursions Test'"),
            Test::RandomExcursionsVariant => write!(f, "'Random Excursions Variant Test'"),
        }
    }
}

/// Struct for "Longest Run of Ones in a Block" test
#[derive(Debug)]
pub struct LongestRunConfig<'a> {
    pub block_size_m: usize,
    pub number_of_blocks: usize,
    pub thresholds: (i32, i32),
    pub pi_values: &'a [f64],
}

impl<'a> LongestRunConfig<'a> {
    pub fn create(
        block_size_m: usize,
        number_of_blocks: usize,
        thresholds: (i32, i32),
        pi_values: &'a [f64],
    ) -> Self {
        LongestRunConfig {
            block_size_m,
            number_of_blocks,
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
