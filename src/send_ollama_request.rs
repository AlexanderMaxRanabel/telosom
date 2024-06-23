use reqwest::{Client};
use serde::{Serialize};

/*#[derive(Deserialize)]
struct OllamaResponse {
    model: String,
    created_at: String,
    response: String,
    done: bool,
}*/

#[derive(Serialize)]
struct TelosomRequest {
    model: String,
    prompt: String,
    stream: bool,
}

pub async fn send_request(input: &str, model: &str) -> Result<(), reqwest::Error> {
    let string_input: String = input.to_string();
    let string_model: String = model.to_string();
    let req_json = TelosomRequest {
        model: string_model,
        prompt: string_input,
        stream: false.into(),
    };

    let client = Client::new();

    let telosom_response = client
        .post("http://localhost:11434/api/generate")
        .json(&req_json)
        .send()
        .await?;

    if telosom_response.status().is_success() {
        let response_body: serde_json::Value = telosom_response.json().await?;
        if let Some(field) = response_body.get("response") {
            println!("{}", field);
        }
    } else {
        eprintln!("Failed to get the code")
    }

    Ok(())
}
