#![allow(dead_code, unused_variables)]
use std::{env, fs, path::PathBuf};

use chrono::{DateTime, Utc};
use rusqlite::{named_params, Connection, ToSql};

use crate::{
    error::{err, Result},
    APP,
};

#[derive(Debug)]
pub struct Cache {
    conn: Connection,
}

#[derive(Debug)]
pub struct CacheEntry {
    url: String,
    pub created_at: DateTime<Utc>,
    pub max_age: Option<u32>,
    last_modified: DateTime<Utc>,
    pub content: String,
}

impl Cache {
    /// Connect to a database and return a handle to perform caching
    /// operations.
    pub fn new() -> Result<Self> {
        Self::with_base_dir(None)
    }

    pub fn with_base_dir(base_dir: Option<PathBuf>) -> Result<Self> {
        let db_path = match base_dir {
            Some(dir) => dir,
            None => match env::var("XDG_DATA_HOME") {
                Ok(dir) => PathBuf::from(dir).join(APP),
                Err(_) => match env::var("HOME") {
                    Ok(dir) => PathBuf::from(dir).join(".local").join("share").join(APP),
                    Err(_) => PathBuf::from("/tmp").join(APP),
                },
            },
        };

        // Make sure the database directory exists
        let db_path = match fs::create_dir_all(&db_path) {
            Ok(_) => Some(db_path),
            Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => Some(db_path),
            Err(_) => None,
        };

        // Open the database, which creates a new db file if needed
        let conn = match db_path {
            Some(mut db_path) => {
                db_path = db_path.join(APP);
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

    pub fn get<T>(&self, url: T) -> Result<Option<CacheEntry>>
    where
        T: AsRef<str> + ToSql,
    {
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

        if let Some(row) = rows.next() {
            let row = row?;
            Ok(Some(row))
        } else {
            Ok(None)
        }
    }

    pub fn insert<T>(
        &mut self,
        url: T,
        max_age: Option<u32>,
        last_modified: DateTime<Utc>,
        content: &str,
    ) -> Result<()>
    where
        T: AsRef<str> + ToSql,
    {
        let sql = "\
            insert into cache(url, created_at, max_age, last_modified, content)
            values(:url, :created_at, :max_age, :last_modified, :content)
            on conflict(url) do update set
                created_at=:created_at, max_age=:max_age,
                last_modified=:last_modified, content=:content";
        self.conn.execute(
            sql,
            named_params! {
                ":url": url,
                ":created_at": Utc::now(),
                ":max_age": max_age,
                ":last_modified": last_modified,
                ":content": content,
            },
        )?;

        Ok(())
    }

    pub fn db_version(&self) -> Result<u32> {
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
    conn.execute_batch(
        "\
        create table cache(
            url text unique,
            created_at datetime,
            max_age int,
            last_modified datetime,
            content text);
        create table version(id int);
        insert into version values(1);
        ",
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use tempfile::{tempdir, TempDir};

    /// Create a cache database in a temporary directory for testing.
    fn tempcache() -> (Cache, TempDir) {
        let temp_dir = tempdir().unwrap();

        // Cache should create new database with version = 1
        let cache = Cache::with_base_dir(Some(temp_dir.path().to_path_buf())).unwrap();
        assert_eq!(cache.db_version().unwrap(), 1);

        (cache, temp_dir)
    }

    #[test]
    fn cache_works() {
        let (mut cache, _temp_dir) = tempcache();
        cache
            .insert("mock.url", Some(888), Utc::now(), "content")
            .unwrap();

        let cached_page = cache.get("mock.url").unwrap().unwrap();
        assert_eq!(cached_page.url, "mock.url".to_string());
    }

    #[test]
    fn db_initializes() {
        let (_cache, temp_dir) = tempcache();
        let mut db_path = PathBuf::from(temp_dir.path());

        // Confirm full database path
        db_path = db_path.join(APP);
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
        let cache = Cache::with_base_dir(Some(temp_dir.path().to_path_buf())).unwrap();
        assert_eq!(cache.db_version().unwrap(), 2);
    }

    #[test]
    fn max_age_works() {
        let (mut cache, _temp_dir) = tempcache();

        // With max-age
        cache
            .insert("url_888", Some(888), Utc::now(), "content")
            .unwrap();
        let cached_page = cache.get("url_888").unwrap().unwrap();
        assert_eq!(Some(888), cached_page.max_age);

        // Without max-age
        cache
            .insert("url_None", None, Utc::now(), "content")
            .unwrap();
        let cached_page = cache.get("url_None").unwrap().unwrap();
        assert_eq!(None, cached_page.max_age);
    }
}
