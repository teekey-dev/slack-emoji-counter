mod models;

use reqwest::Client;
use reqwest::header::*;
use models::*;

#[tokio::main]
async fn main() {
    println!("{:?}", channels().await[0]);
}

async fn channels() -> Vec::<Channel> {
    let client = Client::new();
    let response = client.get("https://slack.com/api/conversations.list")
                         .headers(headers())
                         .query(&[("limit", "1000"), ("exclude_archived", "true")])
                         .send()
                         .await
                         .unwrap();

    let conversation_list_response = response.json::<ConversationListResponse>().await.unwrap();

    conversation_list_response.channels
}

fn headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/x-www-form-urlencoded".parse().unwrap());
    headers.insert(AUTHORIZATION, "Bearer ".parse().unwrap());

    headers
}

