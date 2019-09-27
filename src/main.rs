use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let mut output_line = vec![];
        let mut i = 0;

        for character in line.unwrap().chars() {
            if i % 2 == 0 {
                output_line.push(character.to_ascii_uppercase())
            } else {
                output_line.push(character.to_ascii_lowercase())
            }
            i += 1;
        }

        for character in output_line {
            print!("{}", character);
        }
        print!("\n");
    }
}
