//! This module contains useful functions to support the statistical tests from the NIST suite.

use crate::customtypes;
use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Evaluate passed bit string.
///
/// # Arguments
///
/// test_name - The name of the test the evaluation is made for
/// bit_string - The bit string to evaluate
/// recommended_size - Log a warning if passed bit string has not recommended size
///
/// # Return
///
/// Ok(length) - Return length of bit string if everything is okay
/// Err(err) - Some error occured
pub fn evaluate_bit_string(
    test_name: customtypes::Test,
    bit_string: &str,
    recommended_size: usize,
) -> Result<usize> {
    log::trace!("utils::evaluate_bit_string()");

    // check validity of passed bit string
    if bit_string.is_empty() || bit_string.chars().any(|c| c != '0' && c != '1') {
        anyhow::bail!(
            "{}: Bit string is either empty or contains invalid character(s)",
            test_name
        );
    }

    let length = bit_string.len();
    log::debug!("{}: Bit string has the length {}", test_name, length);

    // If bit string has not the recommended size, it is not an error but log a warning anyways
    if length < recommended_size {
        log::warn!(
            "Recommended size for {} is at least {} bits. Consider imprecision when calculating p-value",
            test_name,
            recommended_size
        );
    }

    Ok(length)
}

/// Convert a given vector of hexadecimal bytes into a bit string.
///
/// # Arguments
///
/// hex_bytes - A byte vector to be converted
///
/// # Return
///
/// Ok(bit_string) - The bit string converted from given hex string
/// Err(err) - Some error occured
pub fn hex_bytes_to_bit_string(hex_bytes: Vec<u8>) -> Result<String> {
    log::trace!("utils::hex_bytes_to_bit_string()");

    // check if given vector is empty or not
    if hex_bytes.is_empty() {
        anyhow::bail!("No hexadecimal bytes to convert passed!");
    }

    // now convert hex bytes to bit string
    let bit_string = hex_bytes
        .iter()
        .flat_map(|&byte| {
            (0..8)
                .rev()
                .map(move |i| if byte & (1 << i) == 0 { '0' } else { '1' })
        })
        .collect();

    log::info!(
        "Successfully converted {} hex bytes to bit string",
        hex_bytes.len()
    );

    Ok(bit_string)
}

/// Read file containing already generated random numbers (either as hex bytes or as bit string).
///
/// # Arguments
///
/// file_path - The path to the file containing random bytes
///
/// # Return
///
/// Ok(bit_string) - The read bit string
/// Err(err) - Some error occured
pub fn read_random_numbers(file_path: &str) -> Result<String> {
    log::trace!("utils::read_random_numbers()");

    // open the file
    let file =
        File::open(file_path).with_context(|| format!("Failed to open file '{}'", file_path))?;

    // read the contents of the file into a string
    // if the random number is separated into multiple lines, concatenate them into one line
    let mut random_string = String::new();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line {
            random_string.push_str(&line);
        }
    }

    // remove any whitespace characters from the string
    random_string.retain(|c| !c.is_whitespace());

    // now decide whether we do have hexadecimal bytes or binary string
    let bit_string = if random_string.chars().all(|c| c == '0' || c == '1') {
        random_string
    } else if random_string.chars().all(|c| c.is_ascii_hexdigit()) {
        // parse the hexadecimal string into bytes
        let random_bytes = hex::decode(&random_string)
            .map_err(|e| anyhow::anyhow!("Failed to parse hexadecimal string: {}", e))?;
        // now convert to bit string
        hex_bytes_to_bit_string(random_bytes)?
    } else {
        anyhow::bail!(
            "File '{}' neither contains valid hex bytes nor valid bit string!",
            file_path
        );
    };

    log::info!("Successfully read {} random bits", bit_string.len());

    Ok(bit_string)
}

/// Untar a given archive in specific destination.
///
/// # Arguments
///
/// archive_name - Archive to be decompressed
/// dest - Destination directory to unpack the archive contents
///
/// # Return
///
/// Ok() - Successfully unpacked archive
/// Err(err) - Some error occured
pub fn untar_archive(archive_name: &str, dest: &str) -> Result<()> {
    log::trace!("utils::untar_archive()");

    // check whether archive and destination exist
    if !std::path::Path::new(archive_name).exists() {
        anyhow::bail!("Archive '{}' does not exist!", archive_name);
    }
    if !(std::path::Path::new(dest).exists() && std::path::Path::new(dest).is_dir()) {
        anyhow::bail!(
            "Destination path '{}' neither exists nor is a directory",
            dest
        );
    }

    // now try to untar the archive
    let file = File::open(archive_name)
        .with_context(|| format!("Failed to open archive '{}'", archive_name))?;
    let decompressed = flate2::read::GzDecoder::new(file);

    let mut archive = tar::Archive::new(decompressed);
    archive
        .unpack(dest)
        .with_context(|| format!("Failed to unpack archive '{}'", archive_name))?;

    log::debug!(
        "Successfully unpacked archive '{}' to '{}'",
        archive_name,
        dest
    );

    Ok(())
}
