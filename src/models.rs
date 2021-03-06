use super::schema::posts;
use serde::{Deserialize, Serialize};

#[derive(Queryable)]
#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a>{
    pub title: &'a str,
    pub body: &'a str,
}
