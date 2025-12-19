//! Supporting utility library.

use crate::errors::PyLineError;

/// Converts a number of bytes into human-readable size units.
pub fn format_file_size(bytes: u64) -> Result<String, PyLineError> {
    const UNITS: &[(&str, u64)] = &[
        ("б", 1),
        ("Кб", 1024),
        ("Мб", 1024_u64.pow(2)),
        ("Гб", 1024_u64.pow(3)),
        ("Тб", 1024_u64.pow(4)),
    ];

    if bytes == 0 {
        return Ok("0 байт".to_string());
    }

    let (label, divisor) = UNITS
        .iter()
        .rev()
        .find(|(_, div)| bytes >= *div)
        .ok_or_else(|| PyLineError::ToolsError {
            description: "Не удалось определить единицу файлового размера".to_string(),
        })?;

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
