use crate::{CodeLanguage, Collector, FileData};
use std::path::{Path, PathBuf};

#[test]
fn file_data_display_format() {
    let file = FileData::new(PathBuf::from("main.rs"), 100);
    let text = format!("{}", file);
    assert!(text.contains("main.rs"));
    assert!(text.contains("100"));
}

#[test]
fn verbose_display_contains_filename_and_size() {
    let file = FileData::new(PathBuf::from("test.py"), 1024);
    let v = file.verbose_display();
    assert!(v.contains("File:"));
    assert!(v.contains("1024"));
}

#[test]
fn remove_dot_from_extensions() {
    let examples = [".py", "rs", ".toml"];
    // Ожидаемый результат после очистки от точек.
    let expected = ["py", "rs", "toml"];

    let c = Collector::new(Path::new("/some/path"), CodeLanguage::Python, false)
        .with_extensions(examples);

    assert!(c.extensions().iter().all(|s| expected.contains(&s)));
}
