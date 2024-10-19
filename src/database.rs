extern crate dotenv;
extern crate rusqlite;

use once_cell::sync::Lazy;
use rusqlite::{params, Connection, Error, OptionalExtension, Row, Transaction};
use std::{collections::HashMap, fs, path::Path};

static DB_PATH: Lazy<String> =
    Lazy::new(|| dotenv::var("SQLITE_DB_PATH").expect("SQLITE_DB_PATH not set"));

pub(crate) fn init() -> Result<(), Error> {
    let connection: Connection =
        Connection::open(DB_PATH.as_str()).expect("Unable to create SQLite database");

    connection.execute(
        r"CREATE TABLE IF NOT EXISTS totals (
            id INTEGER PRIMARY KEY,
            total INTEGER NOT NULL UNIQUE,
            count INTEGER NOT NULL
        )",
        [],
    )?;

    Ok(())
}

pub(crate) fn clear_totals() -> Result<(), Error> {
    let connection: Connection =
        Connection::open(DB_PATH.as_str()).expect("Unable to open SQLite database");

    connection.execute(r"DELETE FROM totals", params![])?;

    Ok(())
}

pub(crate) fn insert(totals_and_counts: &HashMap<usize, usize>) -> Result<(), Error> {
    let mut connection: Connection =
        Connection::open(DB_PATH.as_str()).expect("Unable to open SQLite database");

    let transaction: Transaction<'_> = connection.transaction()?;

    for (&total, &count) in totals_and_counts {
        transaction.execute(
            r"INSERT INTO totals (total, count) VALUES (?, ?)
               ON CONFLICT(total) DO UPDATE SET count = count + excluded.count",
            params![total, count],
        )?;
    }

    transaction.commit()?;
    Ok(())
}

pub(crate) fn get_total_with_highest_count() -> Result<Option<usize>, Error> {
    let connection: Connection =
        Connection::open(DB_PATH.as_str()).expect("Unable to open SQLite database");

    let total_with_highest_count: Option<usize> = connection
        .query_row(
            r"SELECT total FROM totals ORDER BY count DESC LIMIT 1",
            params![],
            |row: &Row<'_>| row.get(0),
        )
        .optional()?;

    Ok(total_with_highest_count)
}

pub(crate) fn delete_db() -> Result<(), Error> {
    let connection: Connection =
        Connection::open(DB_PATH.as_str()).expect("Unable to open SQLite database");

    connection.execute("DROP TABLE IF EXISTS totals", [])?;

    drop(connection);

    if Path::new(DB_PATH.as_str()).exists() {
        fs::remove_file(DB_PATH.as_str()).expect("Unable to delete SQLite file");
    }

    Ok(())
}
