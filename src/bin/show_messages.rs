extern crate chat;
extern crate diesel;

use self::chat::models::*;
use self::diesel::prelude::*;

fn main() {
    use chat::schema::messages::dsl::*;

    let connection = chat::db::establish_connection();
    let results = messages.limit(5).load::<Message>(&connection).expect(
        "Error loading messages",
    );

    println!("Displaying {} messages", results.len());
    for message in results {
        println!("{}:   {}", message.id, message.text);
    }
}
