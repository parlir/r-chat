extern crate chat;
extern crate diesel;

use self::chat::models::User;
use std::io::stdin;

fn main() {
    println!("What would you like your name to be?");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    let name = &name[..(name.len() - 1)]; // Drop the newline character

    let user = User::create(name);
    println!("\nSaved user {} with id {}", user.name, user.id);
}
