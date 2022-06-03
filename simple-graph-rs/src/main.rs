use reqwest::Client;
use serde_json::json;

const QUERY: &str = "
query ($code: ID!) {
    country(code: $code) {
      code
      name
      currency
    }
  }
";

#[tokio::main]
async fn main() {
    let client: Client = Client::new();

    let json = json!({ "query": QUERY, "variables": {"code": "TH"} });

    let response = client
        .post("https://countries.trevorblades.com/")
        .header("Content-Type", "application/json")
        .body(json.to_string())
        .send()
        .await
        .unwrap()
        .text()
        .await;

    let result: serde_json::Value = serde_json::from_str(&response.unwrap()).unwrap();

    println!("{:#?}", result);
}
