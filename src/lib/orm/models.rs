use super::super::schema::*;
use diesel::sql_types::{Array, Text, VarChar};

#[derive(Queryable)]
pub struct Message {
    pub id: i64,
    pub message: String,
}

#[derive(Insertable)]
#[table_name = "messages"]
pub struct NewMessage<'a> {
    pub message: &'a str,
}

#[derive(Queryable)]
pub struct Color {
    pub id: i64,
    pub hexcode: String,
}

#[derive(Insertable)]
#[table_name = "colors"]
pub struct NewColor<'a> {
    pub hexcode: &'a str,
}

#[derive(Queryable)]
pub struct Gradient {
    pub id: i64,
    pub message_id: i64,
    pub color_id: i64,
    pub position: i32,
}

#[derive(Insertable)]
#[table_name = "gradients"]
pub struct NewGradient {
    pub message_id: i64,
    pub color_id: i64,
    pub position: i32,
}

#[derive(Deserialize, Serialize, QueryableByName)]
pub struct Speech {
    #[sql_type = "Text"]
    pub message: String,
    #[sql_type = "Array<VarChar>"]
    pub hexcodes: Vec<String>,
}
