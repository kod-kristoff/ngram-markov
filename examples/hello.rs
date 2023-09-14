use ngram_markov::Brain;

fn main() {
    let mut brain = Brain::default();
    brain.train("Hello, World! Hello, Joe! Hello, World!");

    println!("{}", brain.prompt("Hello,", 16));
}
