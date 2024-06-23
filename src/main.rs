mod send_ollama_request;

use colored::*;
use std::{
    env,
    io::{self, Write},
};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let model = args.get(1).unwrap_or_else(|| {
            std::process::exit(1);
        });

        loop {
            print!("> ");
            io::stdout().flush().expect("Failed to flush stdout");

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let input = input.trim();

            if input == "!exit" {
                break;
            }
            let _ = send_ollama_request::send_request(input, model).await?;
        }
    } else {
        eprintln!("{}: No model has been provided", "Error".red());
    }
    Ok(())
}
