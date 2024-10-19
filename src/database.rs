extern crate dotenv;
extern crate rusqlite;

use rusqlite::{params, Connection, Error, OptionalExtension, Row, Statement, Transaction};
use std::{collections::HashMap, fs, path::Path};

pub(crate) fn init(connection: &Connection) -> Result<(), Error> {
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

pub(crate) fn clear_totals(connection: &Connection) -> Result<(), Error> {
    connection.execute(r"DELETE FROM totals", params![])?;

    Ok(())
}

pub(crate) fn insert(
    connection: &mut Connection,
    totals_and_counts: &HashMap<usize, usize>,
) -> Result<(), Error> {
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

pub(crate) fn get_total_with_highest_count(
    connection: &Connection,
) -> Result<Option<usize>, Error> {
    let total_with_highest_count: Option<usize> = connection
        .query_row(
            r"SELECT total FROM totals ORDER BY count DESC LIMIT 1",
            params![],
            |row: &Row<'_>| row.get(0),
        )
        .optional()?;

    Ok(total_with_highest_count)
}

pub(crate) fn delete_db(connection: Connection, db_path: &str) -> Result<(), Error> {
    connection.execute("DROP TABLE IF EXISTS totals", [])?;

    drop(connection);

    if Path::new(db_path).exists() {
        fs::remove_file(db_path).expect("Unable to delete SQLite file");
    }

    Ok(())
}
