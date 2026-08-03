#[derive(Debug)]
pub struct DB {
    conn: rusqlite::Connection,
}

impl DB {
    pub fn open() -> rusqlite::Result<Self>{
        let sqlite_uri = dotenvy::var("SQLITE_URI").unwrap_or(String::from("db.sqlite"));
        let conn = rusqlite::Connection::open(sqlite_uri)?;
        Ok(Self { conn, })
    }
}
