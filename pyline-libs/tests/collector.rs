use pyline_libs::collector::Collector;
use pyline_libs::errors::PyLineError;
use std::fs::File;
use std::path::PathBuf;
use tokio::fs;
use uuid::Uuid;

async fn setup_test_dir() -> PathBuf {
    let uuid_time = Uuid::new_v4();

    let tmp_dir = std::env::temp_dir().join(format!("collector_test_{}", uuid_time));
    // let _ = fs::remove_dir_all(&tmp_dir).await;
    fs::create_dir_all(&tmp_dir).await.unwrap();
    dbg!(&tmp_dir);

    let test_file = tmp_dir.join("example.py");
    File::create(&test_file).unwrap();

    // Excluded file
    let excluded_file = tmp_dir.join("README.md");
    File::create(&excluded_file).unwrap();

    // Dot directory
    let dot_dir = tmp_dir.join(".git");
    fs::create_dir_all(&dot_dir).await.unwrap();
    File::create(dot_dir.join("hidden.py")).unwrap();

    tmp_dir
}

#[tokio::test]
async fn test_basic_collection() -> Result<(), PyLineError> {
    let root = setup_test_dir().await;

    let files = Collector::new(&root)
        .extensions(["py"])
        .ignore_dot_dirs(true)
        .exclude_files(["README.md"])
        .complete()
        .await?;

    assert_eq!(files.len(), 1);
    assert!(files[0].path.ends_with("example.py"));

    Ok(())
}

#[tokio::test]
async fn test_include_dot_dirs() -> Result<(), PyLineError> {
    let root = setup_test_dir().await;

    let files = Collector::new(&root)
        .extensions(["py"])
        .ignore_dot_dirs(false)
        .complete()
        .await?;

    // now we should see file from .git too
    assert_eq!(files.len(), 2);

    let collected_files: Vec<_> = files
        .iter()
        .map(|f| f.path.file_name().unwrap().to_str().unwrap())
        .collect();
    assert!(collected_files.contains(&"example.py"));
    assert!(collected_files.contains(&"hidden.py"));

    Ok(())
}

#[tokio::test]
async fn test_exclude_dirs_works() -> Result<(), PyLineError> {
    let root = setup_test_dir().await;

    let subdir = root.join("node_modules");
    fs::create_dir_all(&subdir).await?;
    let file = subdir.join("ignoreme.py");
    File::create(&file)?;

    let files = Collector::new(&root)
        .extensions(["py"])
        .exclude_dirs(["node_modules"])
        .ignore_dot_dirs(false)
        .complete()
        .await?;

    assert!(!files.iter().any(|f| f.path.ends_with("ignoreme.py")));

    Ok(())
}

#[tokio::test]
#[should_panic(expected = "Cannot exclude dot-directories")]
async fn test_exclude_dot_dir_panics() {
    let root = setup_test_dir().await;

    // Этот вызов должен паниковать из-за .git в exclude_dirs
    Collector::new(&root).exclude_dirs([".git"]);
}
