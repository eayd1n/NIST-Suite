mod binary_matrix_rank;
mod constants;
mod cumulative_sums;
mod customtypes;
mod dft_spectral;
mod frequency_block;
mod frequency_monobit;
mod logger;
mod longest_run;
mod non_overlapping_template;
mod overlapping_template;
mod runs;
mod tests;
mod utils;

use anyhow::Result;

fn main() -> Result<()> {
    logger::init_logger("Trace")?;
    Ok(())
}
