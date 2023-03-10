use std::time::{Duration, Instant};

use postgres::{Client, Error};
use utils::{get_db_client, get_query};

fn main() -> Result<(), Error> {
    let mut client: Client = get_db_client();

    let seed_query: String = get_query("seed.sql");

    let now: Instant = Instant::now();
    client.batch_execute(&seed_query).unwrap();

    let elapsed: Duration = now.elapsed();
    println!("Seeding Elapsed: {:.2?}", elapsed);

    Ok(())
}
