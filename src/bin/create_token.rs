extern crate chat;
extern crate diesel;

use self::chat::models::Token;

fn main() {
    let token = Token::create(1);
    println!("Saved token {} with id {}", token.token, token.id);
}
