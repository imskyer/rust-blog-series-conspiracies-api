use diesel;
use diesel::prelude::*;
use diesel::sql_types::{Bool, Integer, Text};
use diesel::expression::dsl::sql;
use wiki::{WikiPage, LinkProcessed, CategoryToPage};
use schema::{conspiracies, categories_to_pages};
use schema::links_processed::dsl::*;
use actix_web::*;

/// adds a new record to the conspiracies table, returns QueryResult<usize>
pub fn add_conspiracy(conn: &SqliteConnection, conspiracy: &WikiPage) -> QueryResult<usize> {
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

/// adds a new record to the cateories_to_pages table, returns the number of categories inserted
/// or an error string
pub fn add_categories(conn: &SqliteConnection, categories: Vec<CategoryToPage>) -> Result<i32,String> {
    let mut i = 0;
    for cat_to_page in categories.into_iter() {
        match diesel::insert_into(categories_to_pages::table)
            .values(cat_to_page)
            .execute(conn) {
          Err(e) => println!("ERROR ADDING CATEGORY {}", e),
          Ok(_) => i += 1
        };
        
    }

    Ok(i)
}

pub fn get_categories(conn: &SqliteConnection, page_number: i64) -> Result<Vec<String>, String> {
    use schema::conspiracies::dsl::*;
    let page_count: i64 = 25;
    let offset = page_count * page_number;

    let q_stmt = format!("SELECT category FROM categories order by category LIMIT {} OFFSET {};", page_count, offset);
    let query = sql::<(Text)>(&q_stmt);
    Ok(query.load::<String>(conn).expect("Can't query links_processed"))
}

pub fn get_conspiracies(conn: &SqliteConnection, page_number: i64) -> Result<Vec<WikiPage>, String> {
    use schema::conspiracies::dsl::*;
    let page_count: i64 = 25;

    match conspiracies.limit(25).offset(page_count * page_number).load::<WikiPage>(conn) {
        Ok(c) => Ok(c),
        Err(e) => Err(format!("ERROR: {}", e))
    }
}

pub fn get_conspiracy_by_id(conn: &SqliteConnection, id: &str) -> Result<WikiPage, String> {
    use schema::conspiracies::dsl::*;
    match conspiracies.filter(page_id.eq(id.to_string())).first::<WikiPage>(conn) {
        Ok(c) => Ok(c),
        Err(e) => Err(format!("ERROR: {}", e))
    }
}

pub fn mark_link_as_processed(conn: &SqliteConnection, link_title: &str) ->Result<usize, diesel::result::Error> {
    let u_stmt = format!("UPDATE links_processed SET processed=1 WHERE title='{}';", link_title.replace("'", "''"));
    let update = sql::<Bool>(&u_stmt);
    update.execute(conn)
}

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
