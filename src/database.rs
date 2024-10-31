extern crate rusqlite;

use rusqlite::{params, Connection, Error, OptionalExtension, Row, Statement, Transaction};
use std::{collections::HashMap, fs, path::Path};

#[inline(always)]
pub(crate) fn init(filename: &str) -> Result<(), Error> {
    let connection: Connection = Connection::open(filename)?;
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

#[inline(always)]
pub(crate) fn clear_totals(filename: &str) -> Result<(), Error> {
    let connection: Connection = Connection::open(filename)?;
    connection.execute(r"DELETE FROM totals", params![])?;

    Ok(())
}

#[inline(always)]
pub(crate) fn insert(
    filename: &str,
    totals_and_counts: &HashMap<usize, usize>,
) -> Result<(), Error> {
    let mut connection: Connection = Connection::open(filename)?;
    let transaction: Transaction = connection.transaction()?;

    {
        // Introduce a new block to limit the scope of `statement`
        let mut statement: Statement<'_> = transaction.prepare(
            r"INSERT INTO totals (total, count) VALUES (?, ?)
               ON CONFLICT(total) DO UPDATE SET count = count + excluded.count",
        )?;

        for (&total, &count) in totals_and_counts.iter() {
            statement.execute(params![total, count])?;
        }
        // `statement` goes out of scope and is dropped here
    }

    transaction.commit()?;
    Ok(())
}

#[inline(always)]
pub(crate) fn get_total_with_highest_count(filename: &str) -> Result<Option<usize>, Error> {
    let connection: Connection = Connection::open(filename)?;
    let total_with_highest_count: Option<usize> = connection
        .query_row(
            r"SELECT total FROM totals ORDER BY count DESC LIMIT 1",
            params![],
            |row: &Row<'_>| row.get(0),
        )
        .optional()?;

    Ok(total_with_highest_count)
}

#[inline(always)]
pub(crate) fn delete_db(filename: &str) -> Result<(), Error> {
    let connection: Connection = Connection::open(filename)?;
    connection.execute("DROP TABLE IF EXISTS totals", [])?;

    drop(connection);

    if Path::new(filename).exists() {
        fs::remove_file(filename).expect("Unable to delete SQLite file");
    }

    Ok(())
}
