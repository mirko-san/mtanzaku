use std::io;
use std::io::Write;

pub fn create_all_vec(s:&String) -> Vec<char> {
    let mut vec: Vec<char> = Vec::new();
    let chars = s.trim().chars();

    for s in chars {
        vec.push(s);
    }

    return vec;
}

pub fn stdin() {
    let mut _buffer = String::new();
    print!("Enter a word : ");
    io::stdout().flush().unwrap();

    let stdin = io::stdin();
    stdin.read_line(&mut _buffer).expect("Error getting guess");

    let str = create_all_vec(&_buffer);

    println!("-----");
    for s in str {
        println!("| {} |", s);
    }
    println!("-----");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_all_vec() {
        let s = "hello world!";
        let result = create_all_vec(&s.to_string());
        assert_eq!(result, vec!['h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd', '!']);
    }
}
