use std::time::{Duration, Instant};

use postgres::{Client, Error};
use utils::{get_db_client, get_query, Author};

fn main() -> Result<(), Error> {
    let mut client: Client = get_db_client();

    let list_query: String = get_query("list.sql");

    let now: Instant = Instant::now();

    for row in client.query(&list_query, &[])? {
        let author: Author = Author {
            _id: row.get(0),
            name: row.get(1),
            country: row.get(2),
        };
        println!("Author {} is from {}", author.name, author.country);
    }

    let elapsed: Duration = now.elapsed();

    println!("Listing Elapsed: {:.2?}", elapsed);

    Ok(())
}
