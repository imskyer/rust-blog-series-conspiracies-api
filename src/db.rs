
use diesel;
use diesel::prelude::*;
use wiki::{WikiPage};
use schema::{conspiracies};

/// adds a new record to the conspiracies table
pub fn add_conspiracy(conn: SqliteConnection, conspiracy: WikiPage) -> QueryResult<usize> {
    diesel::insert_into(conspiracies::table)
        .values(conspiracy)
        .execute(&conn)
}

/// creates a connection to a SQLite database
pub fn get_sqlite_connection(database_url: String) -> SqliteConnection {
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}