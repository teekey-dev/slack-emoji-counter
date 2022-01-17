mod models;
use models::*;
use std::collections::HashMap;
use confy;

#[tokio::main]
async fn main() {
    let configuration: Configuration = confy::load_path("configuration.toml").unwrap();

    let slack_client = SlackClient::new(configuration.get_token());
    let channels = slack_client.get_channels().await.channels;
    let channel: Channel = channels.into_iter().find(|channel| channel.name == configuration.get_channel_name()).unwrap();

    let mut cursor = "0".to_string();
    let mut messages: Vec<Message> = vec![];

    for _i in 0..10 {
        let mut conversation_history_response = slack_client.get_messages(&channel.id, 1000, &cursor).await;

        messages.append(&mut conversation_history_response.messages);
        cursor = conversation_history_response.response_metadata.next_cursor;
    }

    let reactions: Vec<Reaction> = messages.into_iter()
                                           .filter(|message| message.reactions.is_some())
                                           .flat_map(|message| message.reactions.unwrap())
                                           .collect();

    let mut reaction_statistics: HashMap<String, u16> = HashMap::new();

    reactions.iter().for_each(|reaction|  {
        if let Some(count) = reaction_statistics.get_mut(&reaction.name) {
            *count += reaction.count;
        } else {
            reaction_statistics.insert(reaction.name.clone(), reaction.count);
        }
    });

    let mut reaction_statistics: Vec<(String, u16)> = reaction_statistics.into_iter().collect();
    reaction_statistics.sort_by(|a, b| b.1.cmp(&a.1));

    reaction_statistics.iter().for_each(|reaction_count| println!("name: {}, count: {}", reaction_count.0, reaction_count.1));
}

