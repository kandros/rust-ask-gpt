#![feature(panic_info_message)]
use std::panic::set_hook;

use dialoguer::{theme::ColorfulTheme, Input};
use model::{GPT3RequestBody, GTP3Response};
use reqwest::blocking::Response;

mod model;

const API_URL: &str = "https://api.openai.com/v1/chat/completions";
fn main() {
    set_hook(Box::new(|info| {
        println!("{:?}", info.message().unwrap());
    }));

    let api_key = std::env::var("OPEN_AI_API_KEY").expect("Could not find env OPEN_AI_API_KEY");

    let prompt: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Ask AI")
        .interact_text()
        .unwrap();

    let result = ask_open_ai(&api_key, &prompt);
    println!("{result}")
}

fn ask_open_ai(api_key: &str, prompt: &str) -> String {
    let body = GPT3RequestBody::new(prompt.to_string());
    let response = send_request(api_key, body);
    let response = extract_response(response);
    response
}

fn extract_response(response: Response) -> String {
    let response = response
        .json::<GTP3Response>()
        .expect("Could not parse response");

    let response = response.get_text();
    response.to_string() /* WHHY TRIM? */
}

fn send_request(api_key: &str, body: GPT3RequestBody) -> Response {
    let client = reqwest::blocking::Client::new();
    let result = client
        .post(API_URL)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body)
        .send()
        .expect("Could not send request");

    result
}
