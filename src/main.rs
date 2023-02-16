use std::io;

mod models;
mod services;

fn main() -> io::Result<()> {
    let results = services::parse_file("create_accounts.txt")?;

    for result in results.iter() {
        println!("{:?}", result);
    }

    Ok(())
}
