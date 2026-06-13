use crate::error::Result;
use crate::tracing;
use std::time::Duration;

pub fn show_warning(seconds_before_sleep: u64) -> Result<()> {
    tracing::info!(
        "Warning: system will sleep in {} seconds",
        seconds_before_sleep
    );
    std::thread::sleep(Duration::from_secs(seconds_before_sleep));
    Ok(())
}
