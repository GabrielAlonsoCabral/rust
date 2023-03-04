use args::EmailArgs;
use structopt::StructOpt;
mod mailgun;
mod args;
mod messager;

fn main() {
    let args: EmailArgs = EmailArgs::from_args();
    let mailgun = mailgun::Mailgun::sset_message( to, body, subject)
}

fn print(message: String) {
    println!("{:?}", message)
}
