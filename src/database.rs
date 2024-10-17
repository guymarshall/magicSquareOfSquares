extern crate dotenv;
extern crate mysql;

use mysql::*;
use once_cell::sync::Lazy;
use prelude::Queryable;
use std::collections::HashMap;

static MYSQL_USER: Lazy<String> =
    Lazy::new(|| dotenv::var("MYSQL_USER").expect("MYSQL_USER not set"));
static MYSQL_PASSWORD: Lazy<String> =
    Lazy::new(|| dotenv::var("MYSQL_PASSWORD").expect("MYSQL_PASSWORD not set"));
static MYSQL_DB: Lazy<String> = Lazy::new(|| dotenv::var("MYSQL_DB").expect("MYSQL_DB not set"));
static OPTS: Lazy<OptsBuilder> = Lazy::new(|| {
    OptsBuilder::new()
        .user(Some(MYSQL_USER.clone()))
        .pass(Some(MYSQL_PASSWORD.clone()))
        .db_name(Some(MYSQL_DB.clone()))
        .ip_or_hostname(Some("localhost"))
        .tcp_port(3306)
});
static POOL: Lazy<Pool> = Lazy::new(|| Pool::new(OPTS.clone()).expect("Unable to create pool"));

pub(crate) fn clear_totals() -> Result<()> {
    let mut pooled_connection: PooledConn = POOL.get_conn().expect("Failed to get connection");

    pooled_connection.exec_drop(r"DELETE FROM totals", ())?;

    Ok(())
}

pub(crate) fn insert(totals_and_counts: &HashMap<usize, usize>) -> Result<()> {
    let mut pooled_connection: PooledConn = POOL.get_conn().expect("Failed to get connection");

    let mut values: Vec<String> = Vec::new();

    for (&total, &count) in totals_and_counts {
        values.push(format!("({}, {})", total, count));
    }

    let value_str: String = values.join(", ");
    let query: String = format!(
        r"INSERT INTO totals (total, count) VALUES {}
           ON DUPLICATE KEY UPDATE count = count + VALUES(count)",
        value_str
    );

    pooled_connection.query_drop(query)?;

    Ok(())
}

pub(crate) fn get_total_with_highest_count() -> Result<Option<usize>> {
    let mut pooled_connection: PooledConn = POOL.get_conn().expect("Failed to get connection");

    let total_with_highest_count: Option<usize> = pooled_connection
        .exec_first(r"SELECT total FROM totals ORDER BY count DESC LIMIT 1", ())?;

    Ok(total_with_highest_count)
}
