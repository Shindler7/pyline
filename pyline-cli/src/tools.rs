//! Miscellaneous helpers for the `pyline-cli` crate.

use anyhow::{Context, Result as AnyhowResult};
use std::{
    io,
    io::Write,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::sleep,
    time::Duration,
};

/// Displays animated dots while `running` remains true.
///
/// Outputs a growing sequence of dots (`.`) at 10 FPS to indicate
/// ongoing activity. Stops when `running` is set to false.
pub(super) fn show_dot(running: Arc<AtomicBool>) -> AnyhowResult<()> {
    const SLEEP_DURATION: Duration = Duration::from_millis(100);

    while running.load(Ordering::Relaxed) {
        {
            let mut stdout = io::stdout().lock();

            print!(".");
            stdout
                .flush()
                .context("Failed to flush stdout during loading animation")?;
        }

        sleep(SLEEP_DURATION);
    }

    println!();
    io::stdout()
        .flush()
        .context("Failed to final flush stdout after animation stopped")?;

    Ok(())
}
