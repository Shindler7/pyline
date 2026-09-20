//! Miscellaneous helpers.

/// Formats a byte count as a human-readable string using binary prefixes
/// (`KB`, `MB`, `GB`, `TB`).
///
/// Values below 10 use one decimal place; larger values are rounded to
/// integers.
///
/// # Examples
///
/// ```
/// use pyline_libs::utils::format_file_size;
///
/// assert_eq!(format_file_size(0), "0 bytes");
/// assert_eq!(format_file_size(1), "1 byte");
/// assert_eq!(format_file_size(2048), "2.0 KB");
/// assert_eq!(format_file_size(10 * 1024 * 1024), "10 MB");
/// ```
pub fn format_file_size(bytes: u64) -> String {
    const UNITS: &[(&str, u64)] = &[
        ("TB", 1 << 40), // 1024^4
        ("GB", 1 << 30), // 1024^3
        ("MB", 1 << 20), // 1024^2
        ("KB", 1 << 10), // 1024
    ];

    if bytes < 1024 {
        return if bytes == 1 {
            "1 byte".to_string()
        } else {
            format!("{bytes} bytes")
        };
    }

    let (label, divisor) = UNITS
        .iter()
        .find(|(_, div)| bytes >= *div)
        .expect("UNITS covers all values >= 1024");

    let size = bytes as f64 / *divisor as f64;

    if size < 10.0 {
        format!("{:.1} {}", size, label)
    } else {
        format!("{:.0} {}", size, label)
    }
}
