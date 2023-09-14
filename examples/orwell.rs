use ngram_markov::Brain;
use std::error::Error;
use std::fs;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut text = String::new();
    let mut file = fs::File::open("data/1984.txt")?;
    file.read_to_string(&mut text)?;

    let mut brain = Brain::default();
    brain.train(&text);

    println!("{}", brain.prompt("It was a", 64));
    Ok(())
}
