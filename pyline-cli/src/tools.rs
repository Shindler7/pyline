//! Tools and utils for pyline.

use std::{
    io::Write,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};
use tokio::time;

/// Displays animated dots while `running` remains true.
///
/// Outputs a growing sequence of dots (`.`) at 10 FPS to indicate
/// ongoing activity. Stops when `running` is set to false.
pub async fn show_dot(running: Arc<AtomicBool>) {
    const SLEEP_DURATION: Duration = Duration::from_millis(100);

    while running.load(Ordering::Relaxed) {
        print!(".");
        Write::flush(&mut std::io::stdout()).unwrap();
        time::sleep(SLEEP_DURATION).await;
    }

    Write::flush(&mut std::io::stdout()).unwrap();
}
