use diesel::prelude::*;

#[derive(Debug, Identifiable, Queryable, Selectable)]
#[diesel(table_name = crate::schema::nested)]
pub struct Nested {
    pub id: i32,
    pub name: String,
    pub parent_id: String,
    pub meta_a_id: i32,
    pub meta_b_id: i32,
}

#[derive(Debug, Identifiable, Queryable, QueryableByName, Selectable)]
#[diesel(table_name = crate::schema::meta_a)]
pub struct MetaA {
    pub id: i32,
    pub data: String,
}

#[derive(Debug, Identifiable, Queryable, QueryableByName, Selectable)]
#[diesel(table_name = crate::schema::meta_b)]
pub struct MetaB {
    pub id: i32,
    pub data: String,
}
