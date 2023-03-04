use std::env;

use mailgun_v3::email::{EmailAddress, Message, MessageBody};
use mailgun_v3::Credentials;

pub struct Mailgun {
    api_key: String,
    domain: String,
    public_key: String,
    login: String,
    password: String,
    port: u32,
    server: String,
    message: Message,
    sender: String,
}

impl Mailgun {
    pub fn new() -> Mailgun {
        Mailgun {
            api_key: get_env("MAILGUN_API_KEY"),
            domain: get_env("MAILGUN_DOMAIN"),
            public_key: get_env("MAILGUN_PUBLIC_KEY"),
            login: get_env("MAILGUN_LOGIN"),
            password: get_env("MAILGUN_PASSWORD"),
            port: get_env("MAILGUN_PORT").parse().unwrap(),
            server: get_env("MAILGUN_SERVER"),
            sender: get_env("MAILGUN_SENDER"),
        }
    }
    pub fn set_message(&mut self, to: Vec<EmailAddress>, body: MessageBody, subject: String) {
        self.message = Message {
            to,
            body,
            subject,
            ..Default::default()
        };
    }

    pub fn send(self) {
        dotenv::dotenv().expect("Failed to read .env file");

        let sender: EmailAddress = EmailAddress::address(get_env("MAILGUN_DOMAIN"));

        let creds: Credentials =
            Credentials::new(get_env("MAILGUN_API_KEY"), get_env("MAILGUN_DOMAIN"));
        let res: Result<mailgun_v3::email::SendResponse, mailgun_v3::ReqError> =
            mailgun_v3::email::send_email(&creds, &sender, self.message);
        println!("{:?}", res);
    }
}

fn get_env(name: &str) -> String {
    env::var(name).expect("Unable to get")
}
