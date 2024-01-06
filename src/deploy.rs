mod train;
mod types;

use nostr_sdk::{prelude::*};
use std::{fs::File, io::Read};
use crate::{train::Train, types::NOSCRIPT_KIND};

#[tokio::main]
async fn main() -> Result<()> {
    let mut file = File::open(".secret")?;
    let mut key = String::new();
    file.read_to_string(&mut key).unwrap();
    let my_keys = Keys::from_sk_str(key.as_str())?;

    let pubkey: String = my_keys.public_key().to_string();
    println!("PubKey: {}", pubkey);

    let client = Client::new(&my_keys);
    client.add_relay("ws://localhost:8080").await?;

    client.connect().await;

    // let metadata = Metadata::new()
    //     .name("algoWriter")
    //     .display_name("Algo Writer")
    //     .about("I write algo and publish it");
    // client.set_metadata(&metadata).await?;

    // Send custom event
    let e = Train::from_local_vecs_to_event().unwrap();
    let event: Event = EventBuilder::new(Kind::Custom(NOSCRIPT_KIND.try_into().unwrap()), "", to_sdk_tags(e.tags)).to_event(&my_keys)?;
    client.send_event(event).await?;

    Ok(())
}

pub fn to_sdk_tags(tags: Vec<Vec<String>>) -> Vec<Tag>{
    let mut sdk_tags: Vec<Tag> = vec![];
    for tag in tags {
        let label = tag.first().unwrap();
        let t = Tag::Generic(TagKind::from(label), tag.iter().skip(1).cloned().collect());
        sdk_tags.push(t);
    }
    sdk_tags
}
