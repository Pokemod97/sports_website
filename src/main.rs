extern crate reqwest;
use serde_json::Value;

fn main() {
    let resp: String = reqwest::get("https://api.chess.com/pub/leaderboards")
        .expect("Could not connect.")
        .text()
        .expect("Could not connect.");
    let response: Value = serde_json::from_str(&resp).unwrap();
    let name: String = response["daily"][0]["name"].to_string().replace("\"", "");
    let score = &response["daily"][0]["score"];
    let url: String = response["daily"][0]["url"].to_string().replace("\"", "");
    println!(
        "Today's top chess player is {} with a score of {}.\nCheck them out at {}",
        name, score, url
    );
}
