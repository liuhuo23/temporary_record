use chrono;
use diesel::prelude::*;

#[derive(Queryable, Debug, PartialEq, Eq)]
pub struct Article {
    pub id: i32,
    pub author: String,
    pub content: String,
    pub tags: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Local>,
    pub updated_at: chrono::DateTime<chrono::Local>,
}
