use iron;
use iron::prelude::*;
use router::Router;

fn index(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "This is the index")))
}

fn something(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "This is the something")))
}

pub fn get_router() -> Router {
    router!(
        index: get "/" => index,
        something: get "/something" => something
    )
}
