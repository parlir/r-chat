extern crate chat;
extern crate diesel;

use self::chat::models::Message;
use std::io::stdin;

fn main() {
    println!("What would you like your text to be?");
    let mut text = String::new();
    stdin().read_line(&mut text).unwrap();
    let text = &text[..(text.len() - 1)]; // Drop the newline character

    let message = Message::create(1, text);
    println!("\nSaved message {} with id {}", text, message.id);
}
