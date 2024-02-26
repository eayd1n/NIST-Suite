mod cumulative_sums;
mod frequency_block;
mod frequency_monobit;
mod logger;
mod longest_run;
mod runs;
mod tests;
mod utils;

use anyhow::Result;

fn main() -> Result<()> {
    logger::init_logger("Trace")?;
    Ok(())
}
