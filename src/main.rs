use std::io::{self, BufRead};
use std::env;


fn main() {
    let mut reversed : bool = false;
    let args : Vec<String> = env::args().collect();
    if args.len() < 1 {
        read_from_stdin(reversed);
    } else {
        for arg in args {
            match arg.as_str() {
                // print help
                "-h" => {
                    show_help();
                    std::process::exit(0);
                },
                // reversed character casing
                "-r" => {
                    reversed = true;
                },
                _ => (),
            }
        }
        read_from_stdin(reversed);
    }
}

fn read_from_stdin(reversed : bool) {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let mut output_line = vec![];
        let mut i = 0;

        for character in line.unwrap().chars() {

            if i % 2 == 0 {
                if reversed {
                    output_line.push(character.to_ascii_lowercase())
                } else {
                    output_line.push(character.to_ascii_uppercase())
                }
            } else {
                if reversed {
                    output_line.push(character.to_ascii_uppercase())
                } else {
                    output_line.push(character.to_ascii_lowercase())
                }
            }
            i += 1;
        }

        for character in output_line {
            print!("{}", character);
        }
        print!("\n");
    }

}

fn show_help() {
    println!("\
        SYNOPSIS:\n\
        \tEither hand over text via standard in:\n\
        \t\techo 'text' | mocktext\n\
        \tOr execute mocktext on it's own and just type text into the terminal:\n\
        \t\t$ mocktext\n\
        \t\t> techno isn't real music\n\
        \t\t  TeChNo iSn'T ReAl mUsIc"
        );
}
