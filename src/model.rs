use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct GPT3RequestBody {
    model: String,
    messages: Vec<PromptMessage>,
    temperature: f32,
    max_tokens: i32,
}

#[derive(Serialize)]
struct PromptMessage {
    role: String,
    content: String,
}

impl GPT3RequestBody {
    pub fn new(prompt: String) -> Self {
        let msg = PromptMessage {
            role: String::from("user"),
            content: prompt,
        };
        GPT3RequestBody {
            messages: vec![msg],
            model: String::from("gpt-3.5-turbo"),
            temperature: 0.5,
            max_tokens: 50,
        }
    }
}

#[derive(Deserialize)]
pub struct GTP3Response {
    // id: String,
    // object: String,
    // created: i32,
    // model: String,
    // usage: Usage,
    choices: Vec<Choice>,
}

// #[derive(Deserialize)]
// struct Usage {
// prompt_tokens: i32,
// completion_tokens: i32,
// total_tokens: i32,
// }
#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
    // finish_reason: Option<String>,
    // index: i32,
}

#[derive(Deserialize)]
struct ResponseMessage {
    // role: String,
    content: String,
}

impl GTP3Response {
    pub fn get_text(&self) -> String {
        match self.choices.first() {
            Some(choice) => choice.message.content.clone(),
            None => panic!("not handled"),
        }
    }
}
