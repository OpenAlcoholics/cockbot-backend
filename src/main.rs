#![feature(proc_macro_hygiene, decl_macro)]

extern crate cockbot_backend;
extern crate diesel;
#[macro_use]
extern crate rocket;

use diesel::Connection;
use diesel::pg::PgConnection;

use cockbot_backend::database::cocktail::Cocktail;
use cockbot_backend::database::Constraints;

fn main() {
    let database_default = "postgres://postgres:password@localhost/cockbot";
    let connection = PgConnection::establish(database_default)
        .expect("Failed to create database connection");

    rocket::ignite()
        // .attach(PrimaryDb::fairing())
        // .manage(routes::Schema::new(QueryRoot, MutationRoot)
        .mount("/", routes![])
        .launch();
}
