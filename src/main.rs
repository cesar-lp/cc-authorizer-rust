use std::io;

mod models;
mod services;

fn main() -> io::Result<()> {
    let results = services::parse_file("inactive_card.txt")?;

    for result in results.iter() {
        println!("{:?}", result);
    }

    Ok(())
}
