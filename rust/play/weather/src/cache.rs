#![allow(dead_code, unused_variables)]
use std::{env, fs, path::PathBuf};

use chrono::{DateTime, Duration, Utc};
use rusqlite::Connection;

use crate::{
    error::{err, Result},
    types::Url,
    APP_NAME,
};

struct Cache {
    conn: Connection,
}

impl Cache {
    /// Connect to a database and return a handle to perform
    /// caching operations.
    fn new() -> Result<Self> {
        // Set database path via environment variable
        let mut env_dir = APP_NAME.to_uppercase();
        env_dir.push_str("_DB");

        // Find the filesystem location of the database
        let db_path = match env::var(&env_dir) {
            Ok(dir) => PathBuf::from(dir),
            Err(_) => match env::var("XDG_DATA_HOME") {
                Ok(dir) => PathBuf::from(dir).join(APP_NAME),
                Err(_) => match env::var("HOME") {
                    Ok(dir) => PathBuf::from(dir)
                        .join(".local")
                        .join("share")
                        .join(APP_NAME),
                    Err(_) => PathBuf::from("/tmp").join(APP_NAME),
                },
            },
        };

        // Make sure the directory exists for a new database
        let db_path = match fs::create_dir_all(&db_path) {
            Ok(_) => Some(db_path),
            Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => Some(db_path),
            Err(_) => None,
        };

        // Open the database, creating a new one if needed
        let conn = match db_path {
            Some(mut db_path) => {
                db_path = db_path.join(APP_NAME);
                db_path.set_extension("db");
                Connection::open(&db_path)?
            }
            None => Connection::open_in_memory()?,
        };

        // Create tables if this is a new database
        if conn.prepare("select max(id) from version").is_err() {
            init_db(&conn)?;
        }

        Ok(Self { conn })
    }

    fn get(&self, url: Url) -> Option<CacheEntry> {
        todo!()
    }

    fn db_version(&self) -> Result<u32> {
        let mut stmt = self.conn.prepare("select max(id) from version")?;
        let mut rows = stmt.query([])?;
        if let Some(row) = rows.next()? {
            Ok(row.get(0)?)
        } else {
            err("Can't determine database version")
        }
    }
}

fn init_db(conn: &Connection) -> Result<()> {
    for sql in &[
        "create table cache(
            url text,
            created_at datetime,
            max_age int,
            last_modified datetime
        )",
        "create table version(id int)",
        "insert into version values(1)",
    ] {
        conn.execute(sql, [])?;
    }
    Ok(())
}

struct CacheEntry {
    created_at: DateTime<Utc>,
    max_age: Duration,
    last_modified: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    use tempfile::tempdir;

    #[test]
    fn init_db() {
        // Override db location with tempdir
        let env_name = format!("{}_DB", APP_NAME.to_uppercase());
        let mut db_path = tempdir().unwrap().into_path();
        env::set_var(&env_name, &db_path);

        // Cache should create new database with version = 1
        let cache = Cache::new().unwrap();
        assert_eq!(cache.db_version().unwrap(), 1);

        // Confirm full database path
        db_path = db_path.join(APP_NAME);
        db_path.set_extension("db");
        let conn = Connection::open(&db_path).unwrap();

        // Check version outside of `Cache::version`
        let mut stmt = conn.prepare("select max(id) from version").unwrap();
        let rows = stmt.query([]);
        match rows {
            Ok(mut rows) => {
                assert_eq!(
                    rows.next().unwrap().unwrap().get::<usize, u32>(0).unwrap(),
                    1
                );
            }
            Err(_) => panic!(),
        }
    }
}
