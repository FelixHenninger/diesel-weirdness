use diesel::sqlite::SqliteConnection;
use diesel::{prelude::*, sql_query, sql_types::Integer};
use dotenvy::dotenv;
use models::{MetaA, MetaB};
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    let mut conn = establish_connection();

    // Entry to retrieve (there would be several of these)
    let nested_id = 2;

    // What we are doing here is:
    // 1. Retrieving a hierarchy from the nested table, by
    //    going through items and retrieving the parent for each
    //    (as long as possible)
    // 2. Attaching optional metadata to each of the items
    //    from two related tables
    let query = sql_query(
        r#"
        WITH RECURSIVE state (
            id,
            parent_id,
            meta_a_id,
            meta_b_id
        ) AS (
            SELECT
                id,
                parent_id,
                meta_a_id,
                meta_b_id
            FROM
                nested
            WHERE
                id = ?
            UNION
            SELECT
                nested.id,
                nested.parent_id,
                nested.meta_a_id,
                nested.meta_b_id
            FROM
                nested,
                state
            WHERE
                nested.id = state.parent_id
        )
        SELECT
            state.*, meta_a.*, meta_b.*
        FROM
            state
            LEFT JOIN meta_a ON state.meta_a_id = meta_a.id
            LEFT JOIN meta_b ON state.meta_b_id = meta_b.id;
        "#,
    )
    .bind::<Integer, _>(nested_id);

    // We expect the resulting table to have two rows,
    // corresponding to nested item 2 and its parent 1.
    // Item 2 comes with a metadata item A,
    // Item 1 comes with a metadata item B.
    // (this is confirmed by running the SQL above manually)
    let res = query.load::<(Option<MetaA>, Option<MetaB>)>(&mut conn);

    // The expected result would this be:
    // [
    //   (Some(MetaA { id: 1, data: "meta_a 1" }), None),
    //   (None, Some(MetaB { id: 1, data: "meta_a 1" }))
    // ]
    // Instead, it is:
    // [
    //   (Some(..), Some(..))
    //   (None, None)
    // ]
    println!("result: {:?}", res);
}
