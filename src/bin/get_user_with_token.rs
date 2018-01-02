extern crate chat;
extern crate diesel;

use self::chat::models::Token;
use std::io::stdin;

fn main() {
    println!("what's the token string?");
    let mut token = String::new();
    stdin().read_line(&mut token).unwrap();
    let token = &token[..(token.len() - 1)]; // Drop the newline character

    match Token::get_user(&token) {
        Ok(user) => println!("\nSaved user {} with id {}", user.name, user.id),
        Err(_) => println!("Couldn't get user with the given token"),
    }

}
