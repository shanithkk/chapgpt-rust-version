use dotenv::dotenv;
use reqwest::header;
use serde_derive::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};
use std::env;
use std::io::{stdin, stdout, Write};

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Responses {
    id: Option<String>,
    object: Option<String>,
    created: Option<u64>,
    model: Option<String>,
    choices: Vec<Choices>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Choices {
    text: String,
    index: u8,
    logprobs: Option<u8>,
    finish_reason: String,
}

#[derive(Serialize, Debug)]
struct Request {
    prompt: String,
    max_tokens: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    let client = reqwest::Client::new();
    let uri = "https://api.openai.com/v1/engines/text-davinci-001/completions";
    let oai_token: String = env::var("OAI_TOKEN").unwrap();
    println!("{esc}c", esc = 27 as char);

    loop {
        print!("=>");
        stdout().flush().unwrap();
        let mut user_text = String::new();

        stdin()
            .read_line(&mut user_text)
            .expect("Failed to read line");
        println!("");
        let spinner = Spinner::new(&Spinners::Dots12, "\t\t Open Ai ChatGPT".into());
        let request = Request {
            prompt: format!("{}", user_text),
            max_tokens: 1000,
        };
        let req = client
            .post(uri)
            .header(header::CONTENT_TYPE, "application/json")
            .header("Authorization", &format!("Bearer {} ", oai_token))
            .json(&serde_json::to_value(request).unwrap())
            .send()
            .await
            .unwrap();
        let json: Responses = serde_json::from_value(req.json().await.unwrap())?;
        spinner.stop();
        println!("");
        println!("{}", json.choices[0].text);
    }
}
