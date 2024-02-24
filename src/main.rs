mod frequency_monobit;
mod frequency_block;
mod logger;
mod tests;
mod utils;

use anyhow::Result;

fn main() -> Result<()> {
    logger::init_logger("Trace")?;
    Ok(())
}
