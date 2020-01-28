extern crate cockbot_backend;
extern crate diesel;

use diesel::Connection;
use diesel::pg::PgConnection;

use cockbot_backend::database::cocktail::Cocktail;
use cockbot_backend::database::Constraints;

fn main() {
    let database_default = "postgres://postgres:password@localhost/cockbot";
    let connection = PgConnection::establish(database_default)
        .expect("Failed to create database connection");

    Cocktail::get(Constraints::default(), &connection)
        .unwrap()
        .iter()
        .for_each(|x| {
            println!("{:#?}", x);
        })
}
