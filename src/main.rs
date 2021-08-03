use actix_web::*;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;
use self::models::*;
use serde::{Deserialize, Serialize};


pub mod schema;
pub mod models;

use schema::posts;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post<'a>(conn: &MysqlConnection, title: &'a str, body: &'a str) -> String{

    let new_post = NewPost {
        title,
        body,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
        .unwrap();
    format!("hey")
}

#[get("/")]
async fn hello() -> impl Responder{
    HttpResponse::Ok().body("Hello World!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder{
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder{
    HttpResponse::Ok().body("Hey There!")
}

#[derive(Serialize)]
struct AllPosts{
    posts: Vec<Post>,
}

#[get("/get_posts")]
async fn query_data() -> Result<HttpResponse> {
    
    use self::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");


    let hahe = serde_json::json!(results);
    println!("im so cool");
    Ok(HttpResponse::Ok().json(hahe))

}

#[derive(Deserialize)]
struct Info{
    title: String,
    body: String,
}

#[post("/write_post")]
async fn write_data(info: web::Json<Info>) -> String {
    let connection = establish_connection();
    let title = &info.title;

    let body = &info.body;

     let _post = create_post(&connection, title, &body);
    format!("hey")
}


#[derive(Deserialize)]
struct Update{
    id: i32, 
}


#[put("/update_post")]
async fn update_data(info: web::Json<Update>) -> String{
    println!("hitting update");
    use schema::posts::dsl::*;
    let connection = establish_connection();
    let _post = diesel::update(posts.find(info.id))
        .set(published.eq(true))
        .execute(&connection)
        .unwrap();
    format!("update ;0")
}


#[derive(Deserialize)]
struct Delete{
    id: i32, 
}

#[delete("/delete_post")]
async fn delete_data(info: web::Json<Delete>) -> String{

    println!("hitting delete");
    use schema::posts::dsl::*;

    let connection = establish_connection();
    let _num_deleted = diesel::delete(posts.filter(id.eq(info.id)))
        .execute(&connection)
        .unwrap();
    format!("delete ;0")
}





#[actix_web::main]
async fn main() -> std::io::Result<()> {
	

    HttpServer::new(||{
        App::new()
            .service(hello)
            .service(echo)
            .service(query_data)
            .service(write_data)
            .service(update_data)
            .service(delete_data)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}



