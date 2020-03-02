#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate juniper;
#[macro_use]
extern crate lazy_static;
extern crate regex;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate rocket_http;

pub mod database;
pub mod graphql;
pub mod models;
pub mod routes;
