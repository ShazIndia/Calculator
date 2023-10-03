#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use rocket::response::content::Json;
use rocket_contrib::json::JsonValue;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[get("/books")]
fn books() -> Json<JsonValue> {
    use crate::schema::books::dsl::*;

    let connection = establish_connection();
    let results = books
        .limit(10)
        .load::<Book>(&connection)
        .expect("Error loading books");

    let mut books_json = json![];
    for book in results {
        books_json.push(json!({
            "title": book.title,
            "author": book.author,
            "publisher": book.publisher,
            "year": book.year,
        }));
    }

    Json(json!({ "books": books_json }))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![books])
        .launch();
}
