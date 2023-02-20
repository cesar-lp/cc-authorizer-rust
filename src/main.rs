use std::io;

mod models;
mod services;

fn main() -> io::Result<()> {
    let results = services::parse_file("high_frequency_small_interval.txt")?;

    for result in results.iter() {
        println!("{:?}", result);
    }

    Ok(())
}
