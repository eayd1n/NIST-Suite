//! This module contains useful functions to support the statistical tests.

use anyhow::Result;
/*
pub fn read_hex_string_from_file(file_path: &str) -> Result<String> {
    log::trace!("utils::read_hex_string_from_file()");

    // first of all, check whether file exists or not
    if !std::path::Path::new(file_path).exists() {
        anyhow::bail!("File containing random numbers does not exist: '{}'", file_path);
    }
}
*/
pub fn hex_to_bit_string(hex_string: &str) -> Result<String> {
    log::trace!("utils::hex_to_bit_string()");

    // check if given string is empty or not
    if hex_string.is_empty() {
        anyhow::bail!("No hexadecimal string to convert passed!");
    }

    // remove potential "0x" in the beginning of an hex string
    let hex_string = if hex_string.starts_with("0x") {
        &hex_string[2..]
    } else {
        hex_string
    };

    // validate the passed string contains valid hex bytes
    if !hex_string.chars().all(|c| c.is_ascii_hexdigit()) {
        anyhow::bail!("Invalid hex string: '{}'", hex_string);
    }

    // now convert valid hex string to bit string
    let mut bit_string = String::new();

    for hex_char in hex_string.chars() {
        let byte = hex_char.to_digit(16).unwrap() as u8;
        let binary_str = format!("{:04b}", byte);
        bit_string.push_str(&binary_str);
    }

    log::debug!("Converted '{}' to '{}'", hex_string, &bit_string);
    log::info!("Successfully converted hexadecimal string to bit string");

    Ok(bit_string)
}
