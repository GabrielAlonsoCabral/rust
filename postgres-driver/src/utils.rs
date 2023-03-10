use dotenv::dotenv;
use postgres::{Client, NoTls};
use std::fs;

pub fn get_db_client() -> Client {
    dotenv().ok();

    let conn_string: String = get_env("DB_CONN_STRING");

    let client: Client = Client::connect(conn_string.as_str(), NoTls).unwrap();

    return client;
}

pub fn get_query(query_name: &str) -> String {
    let query_path: String = format!("queries/{}", query_name);
    let error_message: String = format!(
        "Should have been able to read the {} query file",
        query_name
    );

    return fs::read_to_string(query_path).expect(error_message.as_str());
}

pub fn get_env(variable: &str) -> String {
    let error_message: String = format!("{} must be set.", variable);
    return std::env::var(variable).expect(error_message.as_str());
}

pub struct Author {
    pub _id: i32,
    pub name: String,
    pub country: String,
}
