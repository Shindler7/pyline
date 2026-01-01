//! Supporting utility library.

use crate::errors::PyLineError;
use std::time::SystemTime;

/// Converts a number of bytes into human-readable size units.
///
/// Returns a formatted string with appropriate units (bytes, KB, MB, GB, TB).
/// Returns an error if unable to determine the appropriate unit.
///
/// ```
/// use pyline_libs::utils::format_file_size;
///
/// let result = format_file_size(u64::MAX);
/// assert!(result.is_ok()); // Should handle large values correctly
///
/// // Integration with error handling
/// match format_file_size(500) {
///     Ok(size_str) => println!("File size: {}", size_str),
///     Err(err) => eprintln!("Failed to format file size: {}", err),
/// }
/// ```
pub fn format_file_size(bytes: u64) -> Result<String, PyLineError> {
    const UNITS: &[(&str, u64)] = &[
        ("bytes", 1),
        ("Kb", 1024),
        ("Mb", 1024_u64.pow(2)),
        ("Gb", 1024_u64.pow(3)),
        ("Tb", 1024_u64.pow(4)),
    ];

    if bytes == 0 {
        return Ok("0 bytes".to_string());
    }

    let (label, divisor) = UNITS
        .iter()
        .rev()
        .find(|(_, div)| bytes >= *div)
        .ok_or_else(|| PyLineError::scanner_error("failed to determine file size unit"))?;

    if *label == "б" {
        Ok(format!("{} {}", bytes, label))
    } else {
        let size = bytes as f64 / *divisor as f64;
        let result = if size < 10.0 {
            format!("{:.1} {}", size, label)
        } else {
            format!("{:.0} {}", size, label)
        };
        Ok(result)
    }
}

/// Provides the number of seconds since the UNIX epoch, based on system time.
///
/// Panics if an error occurs while obtaining the system time.
#[allow(dead_code)]
pub fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
