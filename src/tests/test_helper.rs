use anyhow::{Context, Result};
use rand::Rng;
use sha3::{Digest, Sha3_512};
use std::io::{BufWriter, Write};

const BITS_PER_LINE: usize = 512;

/// Create samples of good random numbers by using SHA3-512.
///
/// # Arguments
///
/// num_of_samples - Number of samples to be created
/// num_of_bits - Number of bits each sample needs to contain (at least)
/// dest - Destination directory to store the samples
///
/// # Return
///
/// Ok() - Samples could be created successfully
/// Err(err) - Some error occured
pub fn create_good_random_numbers(
    num_of_samples: usize,
    num_of_bits: usize,
    dest: &str,
) -> Result<()> {
    log::trace!("test-helper::create_random_numbers()");

    // create destination directory if it does not exist
    std::fs::create_dir_all(dest)
        .with_context(|| format!("Failed to create destination folder '{}'", dest))?;

    for sample in 0..num_of_samples {
        log::trace!("Processing sample number {}", sample);

        // create a SHA3-512 object and new rng for each sample
        let mut hasher = Sha3_512::new();
        let mut rng = rand::thread_rng();

        // open file for writing the respective sample
        let num_digits = (num_of_samples as f64).log10().ceil() as usize;
        let filename = dest.to_owned()
            + "/"
            + format!("sample_{:0width$}", sample + 1, width = num_digits).as_str();
        let file = std::fs::File::create(&filename)
            .with_context(|| format!("Failed to create file '{}'", &filename))?;
        let mut writer = BufWriter::new(file);

        // generate random data for this sample. Random data is used as input for SHA3-512
        let mut remaining_bits = num_of_bits;
        while remaining_bits >= BITS_PER_LINE {
            let random_data: Vec<u8> = (0..(BITS_PER_LINE / 8)).map(|_| rng.gen()).collect();

            hasher.update(&random_data);
            let hash_result = hasher.finalize_reset();

            for byte in hash_result.iter() {
                writer
                    .write_all(&format!("{:08b}", byte).as_bytes())
                    .with_context(|| {
                        format!("Failed to write random bits into file '{}'", &filename)
                    })?;
            }

            writer.write_all(b"\n")?;

            remaining_bits -= BITS_PER_LINE;
        }
    }

    Ok(())
}
