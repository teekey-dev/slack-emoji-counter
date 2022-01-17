mod models;

use reqwest::{Client, Response};
use reqwest::header::*;
use models::*;

#[tokio::main]
async fn main() {
    let slack_client = SlackClient::new("");
    let channel = &slack_client.get_channels().await.channels[0];
    let message_response = slack_client.get_messages(&channel.id, 2, "0").await;
    let (messages, next_cursor) = (message_response.messages, message_response.response_metadata.next_cursor);

    println!("messages: {:?}", messages);
    println!("next_cursor: {:?}", next_cursor);
}

struct SlackClient {
    client: Client,
    headers: HeaderMap
}

impl SlackClient {
    const CONVERSATION_LIST_ENDPOINT: &'static str = "https://slack.com/api/conversations.list";
    const CONVERSATION_HISTORY_ENDPOINT: &'static str = "https://slack.com/api/conversations.history";

    fn new(token: &str) -> Self {
        let client = Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/x-www-form-urlencoded".parse().unwrap());
        headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse().unwrap());

        SlackClient { client, headers }
    }

    async fn get_channels(&self) -> ConversationListResponse {
        let response = self.get(SlackClient::CONVERSATION_LIST_ENDPOINT, &vec![("limit", "1000"), ("exclude_archived", "true")]).await;

        response.json::<ConversationListResponse>().await.unwrap()
    }

    async fn get_messages(&self, channel_id: &str, limit: u16, cursor: &str) -> ConversationHistoryResponse {
        let response = self.get(SlackClient::CONVERSATION_HISTORY_ENDPOINT, &vec![("channel", channel_id), ("limit", &limit.to_string()), ("cursor", cursor)]).await;

        response.json::<ConversationHistoryResponse>().await.unwrap()
    }

    async fn get(&self, endpoint: &str, query: &Vec::<(&str, &str)>) -> Response {
        self.client.get(endpoint)
                   .headers(self.headers.clone())
                   .query(query)
                   .send()
                   .await
                   .unwrap()
    }
}

