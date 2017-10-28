use rocket::http;
use rocket::request;
use rocket::Outcome;
use rocket::State;
use r2d2;
use r2d2_redis::RedisConnectionManager;

use Pool;

pub struct RedisConnection(pub r2d2::PooledConnection<RedisConnectionManager>);

impl<'a, 'r> request::FromRequest<'a, 'r> for RedisConnection {
    type Error = ();

    fn from_request(request: &'a request::Request<'r>) -> request::Outcome<RedisConnection, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(RedisConnection(conn)),
            Err(_) => Outcome::Failure((http::Status::ServiceUnavailable, ())),
        }
    }
}
