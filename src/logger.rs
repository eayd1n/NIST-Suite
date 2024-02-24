/// Set up the logger. It can be only initialized once.
use anyhow::Result;
use env_logger::Env;

const CRATE_NAME: &str = "rust_nist_suite";
static START: std::sync::Once = std::sync::Once::new();

/// Initialize the logger.
///
/// # Arguments
///
/// loglevel - The loglevel to use
///
/// # Return
///
/// Ok() - Successfully initialized logger
/// Err(err) - Some error occured
pub fn init_logger(loglevel: &str) -> Result<()> {
    let loglevel = match loglevel.to_lowercase().as_str() {
        "off" => log::LevelFilter::Off,
        "error" => log::LevelFilter::Error,
        "warn" => log::LevelFilter::Warn,
        "info" => log::LevelFilter::Info,
        "debug" => log::LevelFilter::Debug,
        "trace" => log::LevelFilter::Trace,
        _ => log::LevelFilter::Debug,
    };

    START.call_once(|| {
        env_logger::Builder::from_env(Env::default().default_filter_or("info"))
            .format_timestamp(None)
            .filter_module(CRATE_NAME, loglevel)
            .init();
    });

    Ok(())
}
