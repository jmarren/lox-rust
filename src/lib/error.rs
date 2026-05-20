use std::process::exit;
use tokio::io;


pub fn bad_input(err: io::Error) {
            println!("error reading line from repl:\n{err}");
            exit(1); 
}

pub fn bad_usage() {
            println!("Usage: jlox [script]");
            exit(64); // command-line usage error status
}


pub fn bad_file_read(path: &str, err: io::Error) {
            println!("error reading file {path}:\n{err}");
            exit(1); 
}

pub fn unexpected_character(line_no: usize) {
            println!("unexpected character on line {line_no}");
            // does not exit so we can find as many errors as possible
}


pub fn unterminated_string(line_no: usize) {
            println!("unterminated string at line {line_no}");
}


