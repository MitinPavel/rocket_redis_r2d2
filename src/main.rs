#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;
extern crate rocket;

use std::default::Default;
use r2d2_redis::RedisConnectionManager;

mod db;
mod controllers;

use controllers::items;

const REDIS_ADDRESS: &'static str = "redis://localhost:6379";

type Pool = r2d2::Pool<RedisConnectionManager>;

fn redis_pool() -> Pool {
    let manager = RedisConnectionManager::new(REDIS_ADDRESS).expect("connection manager");
    let redis_config = Default::default();

    r2d2::Pool::new(redis_config, manager).expect("db pool")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![items::create, items::index])
        .manage(redis_pool())
        .launch();
}
