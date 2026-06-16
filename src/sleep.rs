use crate::config::Config;
use crate::error::Result;
use crate::platform::{Notifier, Platform, PowerControl, SleepType};
use crate::tracing;

pub fn execute_sleep(platform: &dyn Platform, config: &Config) -> Result<()> {
    if config.sleep.warn_before_sleep {
        let cancelled = Notifier::show_sleep_warning(platform, 10, config.sleep.warn_sound_enabled)?;
        if cancelled {
            tracing::info!("Sleep warning cancelled by user activity or button click");
            return Ok(());
        }
    }

    let sleep_type = if config.sleep.hibernate {
        SleepType::Hibernate
    } else {
        SleepType::Sleep
    };

    if config.general.display_off_on_sleep {
        let _ = PowerControl::turn_display_off(platform);
    }

    PowerControl::suspend(platform, sleep_type, true)
}
