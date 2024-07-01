mod send_ollama_request;
mod handle_operations;

use colored::*;
use std::{
    env,
    io::{self, Write},
};



#[tokio::main]
async fn main() -> anyhow::Result<(), reqwest::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let unprocced_model = args.get(1).unwrap_or_else(|| {
            std::process::exit(1);
        });

        let mut model: String = unprocced_model.to_string();

        loop {
            print!("> ");
            io::stdout().flush().expect("Failed to flush stdout");

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let input = input.trim();

            if input.starts_with("!") {
                model = handle_operations::handle_telosom_operations(input).await.expect("Failed to proccess");
            } else {
                let _ = send_ollama_request::send_request(input, model.clone()).await?;
            }
        }
    } else {
        eprintln!("{}: No model has been provided", "Error".red());
    }
    Ok(())
}
