use crate::config::Config;
use crate::error::Result;
use crate::platform::{Platform, SleepType};
use crate::tracing;

pub fn execute_sleep(platform: &dyn Platform, config: &Config) -> Result<()> {
    if config.warn_before_sleep {
        let cancelled = platform.show_sleep_warning(10, config.warn_sound_enabled)?;
        if cancelled {
            tracing::info!("Sleep warning cancelled by user activity or button click");
            return Ok(());
        }
    }

    let sleep_type = if config.hibernate {
        SleepType::Hibernate
    } else {
        SleepType::Sleep
    };

    if config.display_off_on_sleep {
        let _ = platform.turn_display_off();
    }

    platform.suspend(sleep_type, true)
}
