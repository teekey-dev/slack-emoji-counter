use reqwest::{Client, Response};
use reqwest::header::*;
use super::*;

pub struct SlackClient {
    client: Client,
    headers: HeaderMap
}

impl SlackClient {
    const CONVERSATION_LIST_ENDPOINT: &'static str = "https://slack.com/api/conversations.list";
    const CONVERSATION_HISTORY_ENDPOINT: &'static str = "https://slack.com/api/conversations.history";

    pub fn new(token: &str) -> Self {
        let client = Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/x-www-form-urlencoded".parse().unwrap());
        headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse().unwrap());

        SlackClient { client, headers }
    }

    pub async fn get_channels(&self) -> ConversationListResponse {
        let response = self.get(SlackClient::CONVERSATION_LIST_ENDPOINT, &vec![("limit", "1000"), ("exclude_archived", "true"), ("types", "public_channel,private_channel")]).await;

        response.json::<ConversationListResponse>().await.unwrap()
    }

    pub async fn get_messages(&self, channel_id: &str, limit: u16, cursor: &str) -> ConversationHistoryResponse {
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

