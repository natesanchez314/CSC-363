use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    let input_filename : &str = &args[0];
    let filter_word: &str = &args[1];
    let data: String = std::fs::read_to_string(input_filename)?;
    let mut i = 1;
    for line in data.lines() {
        if line.contains(filter_word) {
            println!("{}: {}", i, line);
        }
        i += 1;
    }
    Ok(())
}
