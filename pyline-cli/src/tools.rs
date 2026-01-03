//! Tools and utils for pyline.

use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::time;

/// Displays a terminal spinner while `running` remains true.
///
/// Shows rotating ASCII characters (| / - \) at 10 FPS to indicate
/// ongoing background activity. Stops when `running` is set to false.
#[allow(dead_code)]
pub async fn show_spinner(running: Arc<AtomicBool>) {
    let spinner_chars = ['|', '/', '-', '\\'];
    let mut index = 0;

    while running.load(Ordering::Relaxed) {
        print!("{}", spinner_chars[index]);
        Write::flush(&mut std::io::stdout()).unwrap();
        index = (index + 1) % spinner_chars.len();
        time::sleep(Duration::from_millis(100)).await;
    }

    Write::flush(&mut std::io::stdout()).unwrap();
}

/// Displays animated dots while `running` remains true.
///
/// Outputs a growing sequence of dots (`.`) at 10 FPS to indicate
/// ongoing activity. Stops when `running` is set to false.
pub async fn show_dot(running: Arc<AtomicBool>) {
    while running.load(Ordering::Relaxed) {
        print!(".");
        Write::flush(&mut std::io::stdout()).unwrap();
        time::sleep(Duration::from_millis(100)).await;
    }

    Write::flush(&mut std::io::stdout()).unwrap();
}
