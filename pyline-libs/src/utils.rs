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

#[cfg(test)]
mod tests {
    use super::format_file_size;

    fn run_cases(cases: &[(u64, &str)]) {
        for &(input, expected) in cases {
            let actual = format_file_size(input);
            assert_eq!(
                actual, expected,
                "failed for input={input}: expected `{expected}`, got `{actual}`"
            );
        }
    }

    #[test]
    fn bytes_cases() {
        let cases = [
            (0, "0 bytes"),
            (1, "1 byte"),
            (2, "2 bytes"),
            (1023, "1023 bytes"),
            (1024, "1.0 KB"), // transition to KB
        ];
        run_cases(&cases);
    }

    #[test]
    fn kb_cases() {
        let cases = [
            (1_u64 << 10, "1.0 KB"),
            (1536, "1.5 KB"), // 1.5 * 1024
            (9_u64 << 10, "9.0 KB"),
            (10_u64 << 10, "10 KB"),
        ];
        run_cases(&cases);
    }

    #[test]
    fn mb_cases() {
        let cases = [
            (1_u64 << 20, "1.0 MB"),
            ((5_u64 << 20) + (512_u64 << 10), "5.5 MB"),
            (10_u64 << 20, "10 MB"),
            ((1_u64 << 30) - 1, "1024 MB"), // boundary rounding
        ];
        run_cases(&cases);
    }

    #[test]
    fn gb_cases() {
        let cases = [
            (1_u64 << 30, "1.0 GB"),
            ((3_u64 << 30) + (1_u64 << 29), "3.5 GB"),
            (10_u64 << 30, "10 GB"),
            ((1_u64 << 40) - 1, "1024 GB"), // boundary rounding
        ];
        run_cases(&cases);
    }

    #[test]
    fn tb_cases() {
        let cases = [
            (1_u64 << 40, "1.0 TB"),
            ((7_u64 << 40) + (1_u64 << 39), "7.5 TB"),
            (10_u64 << 40, "10 TB"),
            (u64::MAX, "16777216 TB"),
        ];
        run_cases(&cases);
    }

    #[test]
    fn edge_cases() {
        let cases = [
            // 10239 / 1024 = 9.999..., with {:.1} -> "10.0 KB"
            ((10_u64 << 10) - 1, "10.0 KB"),
            // just under 1 MB, but rounds to 1024 KB
            ((1_u64 << 20) - 1, "1024 KB"),
            // just under 1 GB, but rounds to 1024 MB
            ((1_u64 << 30) - 1, "1024 MB"),
            // 1 PB (2^50) displays as 1024 TB, since PB is not in UNITS
            (1_u64 << 50, "1024 TB"),
        ];
        run_cases(&cases);
    }
}

