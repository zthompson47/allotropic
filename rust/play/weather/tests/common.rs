use tempfile::{tempdir, TempDir};

use wthr::cache::Cache;

#[allow(dead_code)]
pub const API: &str = "https://mock.api";
#[allow(dead_code)]
pub const APP: &str = "test.app";
#[allow(dead_code)]
pub const USER: &str = "user@test.app";

/// Create a cache database in a temporary directory for testing.
pub fn tempcache() -> (Cache, TempDir) {
    let temp_dir = tempdir().unwrap();

    // Cache should create new database with version = 1
    let cache = Cache::with_base_dir(Some(temp_dir.path().to_path_buf())).unwrap();
    assert_eq!(cache.db_version().unwrap(), 1);

    (cache, temp_dir)
}

pub fn json(file: &str) -> String {
    let path = format!(
        "{}/tests/json/{}.json",
        std::env::var("CARGO_MANIFEST_DIR").unwrap(),
        file
    );
    std::fs::read_to_string(path).unwrap()
}
