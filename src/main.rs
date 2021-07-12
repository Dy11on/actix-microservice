use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;
use self::models::*;

pub mod schema;
pub mod models;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
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

#[get("/database")]
async fn query_data() -> impl Responder{
    
    use self::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results{
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }

    HttpResponse::Ok().body("check cmd")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(||{
        App::new()
            .service(hello)
            .service(echo)
            .service(query_data)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}

