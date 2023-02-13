use std::io;

mod services;
mod models;

fn main() -> io::Result<()> {
    let file_contents = services::parse_file("create_accounts.txt")?;
    let results = services::validate_operation(file_contents);

    for result in results.iter() {
        println!("{:?}", result);
    }

    Ok(())
}