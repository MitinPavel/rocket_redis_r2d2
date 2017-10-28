#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;
extern crate rocket;

mod db;
mod controllers;

use db::redis::pool;
use controllers::items;

fn main() {
    rocket::ignite()
        .mount("/", routes![items::create, items::index])
        .manage(pool())
        .launch();
}
