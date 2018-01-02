/// This is the model module
use diesel;
use diesel::prelude::*;
use diesel::result::Error;
use std::time::SystemTime;
use uuid::Uuid;

use super::db::establish_connection;
use super::schema::{messages, users, tokens};

#[derive(Insertable)]
#[table_name = "tokens"]
struct NewToken<'a> {
    pub token: &'a str,
    pub created: SystemTime,
    pub table_users_id: i32,
}

#[derive(Identifiable, Serialize, Deserialize, Associations, Queryable)]
#[belongs_to(User)]
pub struct Token {
    pub id: i32,
    pub token: String,
    pub created: SystemTime,
    pub table_users_id: i32,
}

impl Token {
    pub fn create(user_id: i32) -> Token {
        let new_token = NewToken {
            token: &Uuid::new_v4().to_string(),
            created: SystemTime::now(),
            table_users_id: user_id,
        };
        diesel::insert_into(tokens::table)
            .values(&new_token)
            .get_result(&establish_connection())
            .expect("Error saving new token")
    }

    pub fn get_user(token_string: &str) -> Result<User, String> {
        use super::schema::tokens::dsl::*;
        let connection = establish_connection();
        let user_tokens = tokens
            .filter(token.eq(token_string))
            .load::<Token>(&connection)
            .expect("Error getting user tokens");

        match user_tokens.len() {
            0 => Err(String::from("No user with given token")),
            1 => {
                let u_id = user_tokens[0].table_users_id;
                Ok(User::get(u_id).expect(
                    "couldn't get user when retrieving token",
                ))
            }
            _ => panic!(String::from("Multiple users with given token")),
        }
    }
}

#[derive(Insertable)]
#[table_name = "users"]
struct NewUser<'a> {
    pub name: &'a str,
}

#[derive(Identifiable, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
}

impl User {
    pub fn get(u_id: i32) -> Result<User, Error> {
        use super::schema::users::dsl::*;
        users.find(u_id).first(&establish_connection())
    }
    pub fn create<'a>(name: &'a str) -> User {
        let new_user = NewUser { name };

        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(&establish_connection())
            .expect("Error saving new post")
    }
}

#[derive(Insertable)]
#[table_name = "messages"]
struct NewMessage<'a> {
    pub text: &'a str,
    pub table_users_id: i32,
}

#[derive(Identifiable, Serialize, Deserialize, Associations, Queryable)]
#[belongs_to(User)]
pub struct Message {
    pub id: i32,
    pub text: String,
    pub table_users_id: i32,
}

impl Message {
    pub fn create<'a>(user_id: i32, text: &'a str) -> Message {
        let new_message = NewMessage {
            table_users_id: user_id,
            text: text,
        };

        diesel::insert_into(messages::table)
            .values(&new_message)
            .get_result(&establish_connection())
            .expect("Error saving new post")
    }

    pub fn list() -> Vec<Message> {
        use super::schema::messages::dsl::*;
        messages.load::<Message>(&establish_connection()).expect(
            "couldn't connect",
        )
    }
}
