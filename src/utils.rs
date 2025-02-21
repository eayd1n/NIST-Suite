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

    log::info!(
        "Successfully read {} random bits from '{}'",
        bit_string.len(),
        file_path
    );

    Ok(bit_string)
}

/// Untar a given archive to specific destination.
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

#[cfg(test)]
mod tests {
    use crate::customtypes;
    use crate::logger;
    use crate::utils;
    use crate::test_helper;
    use std::io::Read;

    const LOGLEVEL: &str = "Trace";
    const TEST_NAME: customtypes::Test = customtypes::Test::FrequencyMonobit;
    static RANDOM_BYTES_1: [u8; 6] = [0xab, 0x00, 0xde, 0xd6, 0xf3, 0xc0];
    static RANDOM_BYTES_2: [u8; 9] = [0x00, 0xFF, 0xDE, 0x89, 0xC0, 0x3D, 0xA6, 0xC2, 0xB5];
    const BIT_STRING_1: &str = "101010110000000011011110110101101111001111000000";
    const BIT_STRING_2: &str =
        "000000001111111111011110100010011100000000111101101001101100001010110101";
    const INVALID_BIT_STRING: &str = "010101010101011110101010101010101X0101010101010101010101";
    const NUMBER_OF_BYTES: usize = 100;
    const HEX_BYTES_FILE: &str = "/src/testdata/random_hex_bytes";
    const BIT_STRING_FROM_FILE: &str = "110010101111111010111010101111101101111010101101101111101110111111000000110111101010111111111110";
    const BIT_STRING_FILE: &str = "/src/testdata/random_bit_string";
    const INVALID_CHAR_IN_FILE: &str = "/src/testdata/random_invalid_char";
    const INVALID_FILE: &str = "/non-existing-dir/random_numbers";
    const TEMPLATE_FILE: &str = "/templates/template2.tar.gz";
    const ARCHIVE_DEST_DIR: &str = "/tmp";
    const TEMPLATE_FILE_DEST: &str = "/tmp/template2";

    #[test]
    fn test_hex_bytes_to_bit_string() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        let _ = test_helper::create_bad_random_numbers(500, 1_000_000, "/tmp/samples");

        assert_eq!(
            utils::hex_bytes_to_bit_string(RANDOM_BYTES_1.to_vec()).unwrap(),
            BIT_STRING_1
        );
        assert_eq!(
            utils::hex_bytes_to_bit_string(RANDOM_BYTES_2.to_vec()).unwrap(),
            BIT_STRING_2
        );

        let success: bool;

        // pass empty vector
        match utils::hex_bytes_to_bit_string(Vec::<u8>::new()) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);
    }

    #[test]
    fn test_evaluate_bit_string() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        assert_eq!(
            utils::evaluate_bit_string(TEST_NAME, BIT_STRING_1, BIT_STRING_1.len()).unwrap(),
            BIT_STRING_1.len()
        );
        assert_eq!(
            utils::evaluate_bit_string(TEST_NAME, BIT_STRING_2, BIT_STRING_2.len()).unwrap(),
            BIT_STRING_2.len()
        );

        let mut success: bool;

        // pass empty string
        match utils::evaluate_bit_string(TEST_NAME, "", NUMBER_OF_BYTES) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);

        // pass invalid string
        match utils::evaluate_bit_string(TEST_NAME, INVALID_BIT_STRING, NUMBER_OF_BYTES) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);
    }

    #[test]
    fn test_random_numbers_file() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        // read file containing hex bytes
        let hex_bytes_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + HEX_BYTES_FILE;
        assert_eq!(
            utils::read_random_numbers(&hex_bytes_file).unwrap(),
            BIT_STRING_FROM_FILE
        );

        // read file containing bits
        let bit_string_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + BIT_STRING_FILE;
        assert_eq!(
            utils::read_random_numbers(&bit_string_file).unwrap(),
            BIT_STRING_FROM_FILE
        );

        let mut success: bool;

        // try to read file containing invalid character
        let invalid_char_file = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + INVALID_CHAR_IN_FILE;
        match utils::read_random_numbers(&invalid_char_file) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);

        // try to read non-existing file
        match utils::read_random_numbers(INVALID_FILE) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);
    }

    #[test]
    fn test_untar_archive() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        if std::path::Path::new(TEMPLATE_FILE_DEST).exists() {
            let _ = std::fs::remove_file(TEMPLATE_FILE_DEST);
        }
        assert!(!std::path::Path::new(TEMPLATE_FILE_DEST).exists());

        // untar archive from templates folder
        let template = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + TEMPLATE_FILE;
        assert!(std::path::Path::new(&template).exists());

        let success: bool;
        match utils::untar_archive(&template, ARCHIVE_DEST_DIR) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(success);
        assert!(std::path::Path::new(TEMPLATE_FILE_DEST).exists());

        // check contents of file
        let mut extracted_file =
            std::fs::File::open(TEMPLATE_FILE_DEST).expect("Failed to open extracted file");
        let mut contents = String::new();
        extracted_file
            .read_to_string(&mut contents)
            .expect("Failed to read contents");
        assert!(!contents.is_empty());

        // XXX check some error cases

        // cleanup
        let _ = std::fs::remove_file(TEMPLATE_FILE_DEST);
        assert!(!std::path::Path::new(TEMPLATE_FILE_DEST).exists());
    }
}
