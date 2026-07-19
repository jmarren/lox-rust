mod lib;
use std::{env};




#[tokio::main]
async fn main() {
   let args: Vec<String> = env::args().collect();
    



    match args.len() {
        // if we got an > 1 arg, pass it as file path
        _ if args.len() > 1 => lib::run_file(&args[1]).await,
        // if 1 arg
        1 => lib::run_prompt().await,
        // otherwise error
        _ => lib::error::bad_usage(),
    }
}
