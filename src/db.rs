use diesel;
use diesel::prelude::*;
use diesel::sql_types::{Bool, Integer, Text};
use diesel::expression::dsl::sql;
use models::{Conspiracy, ConspiracyTag, LinkProcessed, NewTag, Tag};
use schema::{conspiracies, conspiracy_tags, tags};
use schema::links_processed::dsl::*;
use actix_web::*;


/// adds a new record to the conspiracies table, returns QueryResult<usize>
pub fn add_conspiracy(conn: &SqliteConnection, conspiracy: &Conspiracy) -> QueryResult<usize> {
    diesel::insert_into(conspiracies::table)
        .values(conspiracy)
        .execute(conn)
}

/// adds a new record to the links_processed table, returns QueryResult<usize>
pub fn add_link_process(conn: &SqliteConnection, link: &LinkProcessed) -> QueryResult<usize> {
    diesel::insert_into(links_processed)
        .values(link)
        .execute(conn)
}

/// adds a new record to the conspiracies table, returns QueryResult<usize>
pub fn add_tag(conn: &SqliteConnection, new_tag: NewTag) -> QueryResult<usize> {
    diesel::insert_into(tags::table)
        .values(new_tag)
        .execute(conn)
}

/// adds a new record to the cateories_to_pages table, returns the number of categories inserted
/// or an error string
pub fn tag_conspiracy(conn: &SqliteConnection, tagged_conspiracy: ConspiracyTag) -> QueryResult<usize> {
    diesel::insert_into(conspiracy_tags::table)
        .values(tagged_conspiracy)
        .execute(conn)
}

/// Returns a Vec with at most 25 conspriacies for the given page_number.
pub fn get_conspiracies(conn: &SqliteConnection, page_number: i32) -> Result<Vec<Conspiracy>, String> {
    use schema::conspiracies::dsl::*;
    let page_count: i32 = 25;

    match conspiracies.limit(page_count as i64).offset((page_count * page_number) as i64).order_by(title.asc()).load::<Conspiracy>(conn) {
        Ok(c) => Ok(c),
        Err(e) => Err(format!("ERROR: {}", e))
    }
}

/// Returns a Vec with at most 25 conspriacies that have the tag specified by the ID for the given page_number.
pub fn get_conspiracies_by_tag_id(conn: &SqliteConnection, page_number: i32, selected_tag_id: i32) -> Result<Vec<Conspiracy>, String> {
    let page_count: i32 = 25;
    let offset = page_number * page_count;
    
    let mut offset_str = String::from("");
    if offset > 0 {
        offset_str = format!("OFFSET {}", offset);
    }
    
    let q_stmt = format!("select c.title, c.page_id, c.summary, c.content, c.background from conspiracies c inner join conspiracy_tags ct on c.page_id = ct.conspiracy_id where ct.tag_id = {} order by c.title limit {} {}", selected_tag_id, page_count, offset_str);
    let query = sql::<(Text, Text, Text, Text, Text)>(&q_stmt);
    match query.load::<Conspiracy>(conn) {
        Ok(rows) => Ok(rows),
        Err(e) => Err(format!("ERROR: {}", e))
    }
}

/// Retrieves a record for the given page_id from the conspiracies table 
pub fn get_conspiracy_by_id(conn: &SqliteConnection, id: &str) -> Result<Conspiracy, String> {
    use schema::conspiracies::dsl::*;
    match conspiracies.filter(page_id.eq(id.to_string())).first::<Conspiracy>(conn) {
        Ok(c) => Ok(c),
        Err(e) => Err(format!("ERROR: {}", e))
    }
}

/// Gets a Vec of the tags that are available
pub fn get_tags(conn: &SqliteConnection, page_number: i32) -> Result<Vec<Tag>, String> {
    use schema::tags::dsl::*;
    let page_count: i32 = 25;
    let offset = page_count * page_number;

    match tags.filter(approved.eq(1)).limit(25).offset((page_count * page_number) as i64).order_by(name.asc()).load::<Tag>(conn) {
        Ok(c) => Ok(c),
        Err(e) => Err(format!("ERROR: {}", e))
    }
}

/// Updates the record in the database to indicate that it has been processed
pub fn mark_link_as_processed(conn: &SqliteConnection, link_title: &str) ->Result<usize, diesel::result::Error> {
    let u_stmt = format!("UPDATE links_processed SET processed=1 WHERE title='{}';", link_title.replace("'", "''"));
    let update = sql::<Bool>(&u_stmt);
    update.execute(conn)
}

/// gets a Vec of links that have yet to be processed and stored in the database
pub fn get_links_to_process(conn: &SqliteConnection, num_links: i32) -> Vec<LinkProcessed> {
    let q_stmt = format!("SELECT title, processed FROM links_processed WHERE processed=0 limit {};", num_links);
    let query = sql::<(Text, Integer)>(&q_stmt);
    query.load::<LinkProcessed>(conn).expect("Can't query links_processed")
}

/// creates a connection to a SQLite database
pub fn get_sqlite_connection(database_url: String) -> SqliteConnection {
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
