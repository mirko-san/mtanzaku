use std::io;
use std::io::Write;

fn main() {
    let mut buffer = String::new();
    print!("Enter a word : ");
    io::stdout().flush().unwrap();

    let stdin = io::stdin();
    stdin.read_line(&mut buffer).expect("Error getting guess");

    println!("-----");
    for s in buffer.trim().chars() {
        println!("| {} |", s);
    }
    println!("-----");
}
