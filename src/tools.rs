//! Общие методы поддержки, утилиты, инструменты.

/// Преобразование байтов в удобновоспринимаемые единицы размера.
pub fn format_file_size_alt(bytes: u64) -> String {
    #[derive(Debug, Clone, Copy)]
    enum FileSizeUnit {
        Bytes,
        Kilobytes,
        Megabytes,
        Gigabytes,
        Terabytes,
        Petabytes,
        Exabytes,
        Zettabytes,
        Yottabytes,
    }

    impl FileSizeUnit {
        fn as_str(&self) -> &'static str {
            match self {
                Self::Bytes => "Б",
                Self::Kilobytes => "КБ",
                Self::Megabytes => "МБ",
                Self::Gigabytes => "ГБ",
                Self::Terabytes => "ТБ",
                Self::Petabytes => "ПБ",
                Self::Exabytes => "ЭБ",
                Self::Zettabytes => "ЗБ",
                Self::Yottabytes => "ЙБ",
            }
        }

        fn divisor(&self) -> f64 {
            match self {
                Self::Bytes => 1.0,
                Self::Kilobytes => 1024.0,
                Self::Megabytes => 1024.0f64.powi(2),
                Self::Gigabytes => 1024.0f64.powi(3),
                Self::Terabytes => 1024.0f64.powi(4),
                Self::Petabytes => 1024.0f64.powi(5),
                Self::Exabytes => 1024.0f64.powi(6),
                Self::Zettabytes => 1024.0f64.powi(7),
                Self::Yottabytes => 1024.0f64.powi(8),
            }
        }
    }

    if bytes == 0 {
        let mut bytes = FileSizeUnit::Bytes;
        return format!("0 {}", bytes.as_str());
    }

    let units = [
        FileSizeUnit::Bytes,
        FileSizeUnit::Kilobytes,
        FileSizeUnit::Megabytes,
        FileSizeUnit::Gigabytes,
        FileSizeUnit::Terabytes,
        FileSizeUnit::Petabytes,
        FileSizeUnit::Exabytes,
        FileSizeUnit::Zettabytes,
        FileSizeUnit::Yottabytes,
    ];

    // Находим подходящую единицу измерения
    let mut suitable_unit = FileSizeUnit::Bytes;
    for &unit in &units {
        if bytes as f64 >= unit.divisor() {
            suitable_unit = unit;
        } else {
            break;
        }
    }

    let size = bytes as f64 / suitable_unit.divisor();
    let unit_str = suitable_unit.as_str();

    // Форматируем в зависимости от размера
    match suitable_unit {
        FileSizeUnit::Bytes => format!("{} {}", bytes, unit_str),
        _ if size < 10.0 => format!("{:.1} {}", size, unit_str),
        _ => format!("{:.0} {}", size, unit_str),
    }
}
