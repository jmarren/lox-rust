mod lib;
use std::{env};



#[tokio::main]
async fn main() {
   let args: Vec<String> = env::args().collect();

    match args.len() {
        _ if args.len() > 1 => lib::run_file(&args[1]).await,
        1 => lib::run_prompt().await,
        _ => lib::error::bad_usage(),
    }
}
