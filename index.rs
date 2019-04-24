use http::{self, Request, Response};
use reqwest::{header, Client};
use serde_json;
use serde_json::{json, Value};
use std::env;
use url::Url;

fn handler(request: Request<String>) -> http::Result<Response<String>> {
    let method = request.method();

    if method.as_str() == "POST" {
        let body: String = request.into_body();
        let data: Value = serde_json::from_str(&body).unwrap();

        if data["action"] == "opened" {
            let token: String = env::var("BOT_TOKEN").unwrap();
            let chat_id: String = env::var("CHAT_ID").unwrap();

            let response = format!(
                "user {} opened a pull request #{}: {}\ncheck here: {}",
                data["pull_request"]["user"]["login"],
                data["number"],
                data["pull_request"]["title"],
                data["pull_request"]["html_url"]
            );

            let url = format!("https://api.telegram.org/bot{}/sendMessage", token);
            let url = Url::parse(&url).expect("Failed to parse URL");

            let body = json!({"chat_id": chat_id, "text": response}).to_string();

            let client = Client::new();

            client
                .post(url)
                .body(body)
                .header(header::CONTENT_TYPE, "application/json")
                .send()
                .unwrap();
        }
    }

    Ok(Response::default())
}
