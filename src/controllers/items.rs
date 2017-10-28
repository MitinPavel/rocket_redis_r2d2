use rocket::http::RawStr;
use db::redis::RedisConnection;
use r2d2;
use r2d2_redis::RedisConnectionManager;
use std::ops::Deref;
use redis::Commands;

const DB_KEY: &'static str = "items";

impl Deref for RedisConnection {
    type Target = r2d2::PooledConnection<RedisConnectionManager>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

//
// $ curl -X POST http://localhost:8000/first
// OK
// $ curl -X POST http://localhost:8000/second
// OK
//
#[post("/<item>")]
fn create(item: &RawStr, connection: RedisConnection) -> String {
    let _: () = connection.lpush(DB_KEY, item.as_str()).unwrap();

    format!("OK")
}

//
// $ curl http://localhost:8000
// second, first
//
#[get("/")]
fn index(connection: RedisConnection) -> String {
    let items: Vec<String> = connection.lrange(DB_KEY, 0, -1).unwrap();

    items.join(", ")
}
