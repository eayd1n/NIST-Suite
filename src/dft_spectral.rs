//! This module performs the Discrete Fourier Transform (Spectral) Test.
//!
//! Description of test from NIST SP 800-22:
//!
//! "The focus of this test is the peak heights in the Discrete Fourier Transform of the sequence. The purpose
//! of this test is to detect periodic features (i.e., repetitive patterns that are near each other) in the tested
//! sequence that would indicate a deviation from the assumption of randomness. The intention is to detect
//! whether the number of peaks exceeding the 95 % threshold is significantly different than 5 %."

use crate::constants;
use crate::customtypes;
use crate::utils;
use anyhow::{Context, Result};
use rustfft::{num_complex::Complex, FftPlanner};

const TEST_NAME: customtypes::Test = customtypes::Test::DFTSpectral;

/// Perform the Discrete Fourier Transform (Spectral) Test by determining the p-value.
///
/// # Arguments
///
/// bit_string - The bit string to be tested for randomness
///
/// # Return
///
/// Ok(p-value) - The p-value which indicates whether randomness is given or not
/// Err(err) - Some error occured
pub fn perform_test(bit_string: &str) -> Result<f64> {
    log::trace!("dft_spectral::perform_test()");

    // capture the current time before executing the actual test
    let start_time = std::time::Instant::now();

    // check if bit string contains invalid characters
    let length = utils::evaluate_bit_string(TEST_NAME, bit_string, constants::RECOMMENDED_SIZE_DFT)
        .with_context(|| "Invalid character(s) in passed bit string detected")?;

    // perform discrete fourier transform on given bit string to retrieve the results
    let spectrum = apply_dft(bit_string, length);

    // calculate height threshold T = sqrt(log(1/0.05) * length)
    let arg: f64 = 1.0 / 0.05;
    let height_threshold = (arg.log10() * (length as f64)).sqrt();
    log::debug!("{}: Height Threshold T = {}", TEST_NAME, height_threshold);

    // calculate expected theoretical (95%) number of peaks N_0 = (0.95 * length) / 2.0
    // also calculate actual observed number N_1 of peaks in M with peaks < T
    let n_0 = 0.95 * (length as f64) * 0.5;

    let mut n_1 = 0.0;
    for value in spectrum.iter().take(length / 2) {
        // calculate modulus defined as |z| = sqrt(a^2 + b^2)
        let modulus = value.norm();
        if modulus < height_threshold {
            n_1 += 1.0;
        }
    }
    log::debug!("{}: N_0 = {}, N_1 = {}", TEST_NAME, n_0, n_1);

    // compute normalized difference d = (N_1 - N_0) / (sqrt((length * 0.95 * 0.05) / 4.0))
    let normalized_diff = (n_1 - n_0) / ((length as f64) * 0.95 * 0.05 * 0.25).sqrt();
    log::debug!(
        "{}: Normalized difference d = {}",
        TEST_NAME,
        normalized_diff
    );

    // finally, compute p-value to decide whether given bit string is random or not
    // Therefore we need the complementary error function: erfc(|normalized_diff| / sqrt(2))
    let p_value = statrs::function::erf::erfc(normalized_diff.abs() / f64::sqrt(2.0));
    log::info!("{}: p-value = {}", TEST_NAME, p_value);

    // capture the current time after the test got executed and calculate elapsed time
    let end_time = std::time::Instant::now();
    let elapsed_time = end_time.duration_since(start_time).as_secs_f64();
    log::info!("{} took {:.6} seconds", TEST_NAME, elapsed_time);

    Ok(p_value)
}

/// Perform the discrete fourier transform on given bit string.
///
/// # Arguments
///
/// bit_string - The bit string the DFT has to be applied on
/// signal_len - The length of the given bit string
///
/// # Return
///
/// abs_real_part - The performed DFT
fn apply_dft(bit_string: &str, signal_len: usize) -> Vec<Complex<f64>> {
    log::trace!("dft_spectral::apply_dft()");

    // convert the bit string into a sequence of real numbers
    let signal: Vec<f64> = bit_string
        .chars()
        .map(|c| if c == '1' { 1.0 } else { -1.0 })
        .collect();
    log::trace!("{}: Signal: {:?}", TEST_NAME, signal);

    // create a planner for FFT with the given signal length
    let mut planner = FftPlanner::<f64>::new();
    let fft = planner.plan_fft_forward(signal_len);

    // convert the real numbers into complex numbers
    let mut spectrum: Vec<Complex<f64>> = signal.iter().map(|&x| Complex::new(x, 0.0)).collect();

    // perform the DFT
    fft.process(&mut spectrum);

    for (i, value) in spectrum.iter().enumerate() {
        log::trace!("{}: Frequency Bin {}: {:?}", TEST_NAME, i, value);
    }

    spectrum
}
