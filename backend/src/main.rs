use actix_web::{get, post, web, App, HttpRequest, HttpServer, Responder, Result};

#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_files as fs;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::env;

use serde::Deserialize;

pub mod schema;

#[derive(Queryable, Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
}

#[derive(Deserialize, Debug)]
pub struct NewPost {
    title: String,
    body: String,
}

pub fn establish_connection() -> SqliteConnection {
    let _ = dotenv::dotenv();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}!", &name)
// }

async fn make_post(post: web::Form<NewPost>) -> Result<impl Responder> {
    Ok(format!("You typed: {:?}", post))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection = establish_connection();

    HttpServer::new(|| {
        App::new()
            .route("/makepost", web::post().to(make_post))
            // .service(fs::Files::new("/", "../frontend/public").index_file("index.html"))
            .service(fs::Files::new("/", "../frontend/build").index_file("index.html"))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
