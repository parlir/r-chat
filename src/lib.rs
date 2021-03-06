#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate iron;
#[macro_use]
extern crate router;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate time;
extern crate uuid;


use iron::prelude::*;

pub mod common;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod schema;
pub mod db;

pub fn run_server() {
    let mut chain = Chain::new(routes::get_router());
    middleware::apply_middleware(&mut chain);
    Iron::new(chain).http("localhost:5000").unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
