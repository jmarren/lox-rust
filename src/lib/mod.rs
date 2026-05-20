// use error;
mod error;
mod token;
mod scanner;


use tokio::io::{self, AsyncWriteExt};

pub async fn run_file(path: &str) {
    match tokio::fs::read_to_string(path).await  {
        Ok(contents) => run(&contents).await,
        Err(e) => error::bad_file_read(path, e),
    }
}

pub async fn run_prompt() {
    let input_reader = std::io::stdin();
    let mut out = tokio::io::stdout();
    loop {
        let _ = out.write_all(b"> ").await;
        out.flush().await.unwrap();
        let mut input = String::new();

        match input_reader.read_line(&mut input) {
            Ok(0) => break, // 0 bytes read indicates EOF
            Ok(_) => run(&input).await,
            Err(e) => error::bad_input(e),
        }
    }
}

async fn run(contents: &str) {

}

