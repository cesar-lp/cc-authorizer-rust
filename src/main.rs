mod account;
mod io;

fn main() -> std::io::Result<()> {
    let results = io::parse_file("multiple_violations.txt")?;

    for result in results.iter() {
        println!("{:?}", result);
    }

    Ok(())
}
