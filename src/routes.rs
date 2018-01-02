use iron;
use iron::prelude::*;
use serde_json;
use router::Router;

use super::models::Message;

pub fn get_router() -> Router {
    router!(
        index: get "/" => index,
        something: get "/create/:message" => create_message
    )
}

fn index(_: &mut Request) -> IronResult<Response> {
    let messages = Message::list();
    let json = serde_json::to_string(&messages).expect("failed json serialize");
    Ok(Response::with((iron::status::Ok, format!("{}", json))))
}

fn create_message(req: &mut Request) -> IronResult<Response> {
    let ref message = req.extensions
        .get::<Router>()
        .unwrap()
        .find("message")
        .unwrap_or("new message.");
    let message = Message::create(1, message);
    let json = serde_json::to_string(&message).expect("failed json serialize");
    Ok(Response::with((iron::status::Ok, format!("{}", json))))
}
