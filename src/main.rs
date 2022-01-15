mod models;
use models::*;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let slack_client = SlackClient::new("");
    let channels = slack_client.get_channels().await.channels;
    let channel: Channel = channels.into_iter().find(|channel| channel.name == "crew-커리어product-개발팀").unwrap();
    let conversation_history_response = slack_client.get_messages(&channel.id, 1000, "0").await;

    let messages = conversation_history_response.messages;
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

    println!("{:?}", reaction_statistics);
}

