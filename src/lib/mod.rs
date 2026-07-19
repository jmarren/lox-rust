


// use error;
pub mod error;
mod token;
mod scanner;
mod expression;
mod parser;
mod literal;



use tokio::io::{AsyncWriteExt};

use crate::lib::{expression::{Binary, HandleBar, Ident}, parser::Parser, scanner::Scanner };

pub async fn run_file(path: &str) {

    // let x = TokenType::Eof;
    // let y = TokenType::If;
    //
    // let m = matches!(x, TokenType::Eof);
    //
    // println!("m = {m}");

    
    let _y = Ident{
        age: 10,
        name: String::from("John"),
        location: String::from("John"),
    };

    // let b = Binary {
    //     left: 
    // };

    let _x = HandleBar{};

    // read the file to a string 
    match tokio::fs::read_to_string(path).await  {
        // run the contents
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
    let mut scanner = Scanner::new(contents.to_string());
    let tokens = scanner.scan();

    let mut parser = Parser::new(tokens.to_vec());

    let expr = parser.parse();

    println!("expr = {:?}", expr);





    
    // println!("tokens: {:?}", tokens);
    
    // macros::nothing!("");

}

