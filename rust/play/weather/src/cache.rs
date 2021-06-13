#![allow(dead_code, unused_variables)]
use std::{env, fs, path::PathBuf};

use chrono::{DateTime, Local, Utc};
use rusqlite::{params, Connection};

use crate::{
    error::{err, Result},
    types::Url,
    APP_NAME,
};

pub struct Cache {
    conn: Connection,
}

impl Cache {
    /// Connect to a database and return a handle to perform
    /// caching operations.
    pub fn new() -> Result<Self> {
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

    #[allow(clippy::ptr_arg)]
    fn get(&self, url: &Url) -> Result<Option<CacheEntry>> {
        let mut stmt = self.conn.prepare(
            "select url, created_at, max_age, last_modified, content
             from cache where url = ?",
        )?;
        let mut rows = stmt.query_map([url], |row| {
            Ok(CacheEntry {
                url: row.get(0)?,
                created_at: row.get(1)?,
                max_age: row.get(2)?,
                last_modified: row.get(3)?,
                content: row.get(4)?,
            })
        })?;

        Ok(Some(rows.next().unwrap().unwrap()))
    }

    #[allow(clippy::ptr_arg)]
    fn insert(
        &mut self,
        url: &Url,
        max_age: u32,
        last_modified: DateTime<Utc>,
        content: &str,
    ) -> Result<()> {
        let sql = "\
            insert into cache(url, created_at, max_age, last_modified, content)
            values(?, ?, ?, ?, ?)
            on conflict(url) do update set
            created_at=?, max_age=?, last_modified=?, content=?";
        self.conn.execute(
            sql,
            params![
                url,
                Local::now(),
                42,
                Local::now(),
                "jello",
                Local::now(),
                42,
                Local::now(),
                "jello"
            ],
        )?;

        Ok(())
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
    // TODO: maybe could use Connection::execute_batch
    for sql in &[
        "create table cache(
            url text unique,
            created_at datetime,
            max_age int,
            last_modified datetime,
            content text
        )",
        "create table version(id int)",
        "insert into version values(1)",
    ] {
        conn.execute(sql, [])?;
    }
    Ok(())
}

struct CacheEntry {
    url: String,
    created_at: DateTime<Utc>,
    max_age: u32,
    last_modified: DateTime<Utc>,
    content: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    use tempfile::tempdir;

    fn tempcache() -> (Cache, PathBuf) {
        // Override db location with tempdir
        let env_name = format!("{}_DB", APP_NAME.to_uppercase());
        let db_dir = tempdir().unwrap().into_path();
        println!("==============>>>{:?}<<<==============", db_dir);

        // TODO: if the var is already set, it silently doesn't set it again..?
        env::remove_var(&env_name);
        env::set_var(&env_name, &db_dir);

        // Cache should create new database with version = 1
        let cache = Cache::new().unwrap();
        assert_eq!(cache.db_version().unwrap(), 1);

        (cache, db_dir)
    }

    #[test]
    fn cache_works() {
        let (mut cache, _) = tempcache();
        cache
            .insert(&"mock.url".to_string(), 880, Utc::now(), "content")
            .unwrap();

        let cached_page = cache.get(&"mock.url".to_string()).unwrap().unwrap();
        assert_eq!(cached_page.url, "mock.url".to_string());
    }

    #[test]
    fn init_db() {
        let (cache, mut db_path) = tempcache();

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

        // Create a new instance of `Cache` to make sure it can reuse the database
        conn.execute("update version set id = ?", [2]).unwrap();
        let cache = Cache::new().unwrap();
        assert_eq!(cache.db_version().unwrap(), 2);
    }
}
