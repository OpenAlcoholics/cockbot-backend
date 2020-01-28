#![feature(proc_macro_hygiene, decl_macro)]

extern crate cockbot_backend;
extern crate diesel;
#[macro_use]
extern crate rocket;

use diesel::Connection;
use diesel::pg::PgConnection;

use cockbot_backend::database::cocktail::Cocktail;
use cockbot_backend::database::Constraints;
use cockbot_backend::database::PrimaryDb;
use cockbot_backend::graphql::{MutationRoot, QueryRoot};
use cockbot_backend::routes;

fn main() {
    rocket::ignite()
        .attach(PrimaryDb::fairing())
        .manage(routes::Schema::new(QueryRoot, MutationRoot))
        .mount("/", routes![
            routes::graphiql,
            routes::get_graphql_handler,
            routes::post_graphql_handler
        ])
        .launch();
}
