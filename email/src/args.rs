use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct EmailArgs {
    #[structopt(short = "f", long = "from", help = "Email sender")]
    pub from: String,
    #[structopt(short = "d", long = "destinary", help = "Email destinary")]
    pub destinary: String,
    #[structopt(short = "t", long = "title", help = "Message title")]
    pub title: String,
    #[structopt(short = "b", long = "body", help = "Message body")]
    pub body: String,
}
