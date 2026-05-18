use std::{cmp::Ordering, env, io, process::exit};
use tokio::io::AsyncWriteExt;


async fn run_file(path: &str) {
    println!("running file");
    match tokio::fs::read_to_string(path).await  {
        Ok(contents) => run(&contents).await,
        Err(e) => file_read_err(path, e),
    }
}

async fn run_prompt() {
    let input_reader = io::stdin();
    let mut out = tokio::io::stdout();
    loop {
        let _ = out.write_all(b"> ").await;
        out.flush().await.unwrap();
        let mut input = String::new();

        match input_reader.read_line(&mut input) {
            Ok(_) => run(&input).await,
            Err(e) => input_err(e),
        }
        if input == "exit\n" {
            break;
        }
    }
}

async fn run(contents: &str) {

}

fn input_err(err: io::Error) {
            println!("error reading line from repl:\n{err}");
            exit(1); 
}

fn usage_err() {
            println!("Usage: jlox [script]");
            exit(64); // command-line usage error status
}


fn file_read_err(path: &str, err: io::Error) {
            println!("error reading file {path}:\n{err}");
            exit(1); 
}


#[tokio::main]
async fn main() {
   let args: Vec<String> = env::args().collect();

    match args.len() {
        _ if args.len() > 1 => run_file(&args[1]).await,
        1 => run_prompt().await,
        _ => usage_err(),
    }



}
