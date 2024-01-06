mod train;
mod types;

use crate::{train::Train, types::NOSCRIPT_KIND};
use base64::{engine::general_purpose, Engine};
use nostr_sdk::prelude::*;
use std::{fs::File, io::Read};

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

    // Send custom event
    let content = read_wasm();
    let e = Train::from_local_vecs_to_event().unwrap();
    let mut tags = to_sdk_tags(e.tags);
    let filter: Filter = Filter::new().kind(Kind::TextNote);
    let id = "computer&internet";
    let description = "a noscript that filter text for computer&internet topic only";
    let filter_tags = create_filter_tag(filter, Some(id.to_string()), Some(description.to_string()));
    for t in filter_tags {
        tags.push(t);
    }
    
    let event: Event = EventBuilder::new(
        Kind::Custom(NOSCRIPT_KIND.try_into().unwrap()),
        content,
        tags,
    )
    .to_event(&my_keys)?;
    client.send_event(event).await?;

    Ok(())
}

pub fn to_sdk_tags(tags: Vec<Vec<String>>) -> Vec<Tag> {
    let mut sdk_tags: Vec<Tag> = vec![];
    for tag in tags {
        let label = tag.first().unwrap();
        let t = Tag::Generic(TagKind::from(label), tag.iter().skip(1).cloned().collect());
        sdk_tags.push(t);
    }
    sdk_tags
}

pub fn read_wasm() -> String {
    let wasm_file_path = "pkg/noscript_bg.wasm";
    let mut wasm_file = File::open(wasm_file_path).expect("Failed to open .wasm file");
    let mut wasm_bytes = Vec::new();
    wasm_file
        .read_to_end(&mut wasm_bytes)
        .expect("Failed to read .wasm file");

    let wasm_base64 = general_purpose::STANDARD.encode(&wasm_bytes);

    println!("Base64-encoded .wasm file:\n{}", wasm_base64);

    return wasm_base64;
}

pub fn create_filter_tag(filter: Filter, id: Option<String>, description: Option<String>) -> Vec<Tag> {
    let mut tags: Vec<Tag> = vec![];

    if filter.ids.len() > 0 {
        let tag = Tag::Generic(
            TagKind::from("ids"),
            filter.ids.iter().map(|id| id.to_string()).collect(),
        );
        tags.push(tag);
    }
    if filter.authors.len() > 0 {
        let tag = Tag::Generic(
            TagKind::from("authors"),
            filter.authors.iter().map(|id| id.to_string()).collect(),
        );
        tags.push(tag);
    }
    if filter.kinds.len() > 0 {
        let tag = Tag::Generic(
            TagKind::from("kinds"),
            filter.kinds.iter().map(|id| id.to_string()).collect(),
        );
        tags.push(tag);
    }
    if filter.limit.is_some() {
        let tag = Tag::Generic(
            TagKind::from("limit"),
            vec![filter.limit.unwrap().to_string()],
        );
        tags.push(tag);
    }
    if filter.since.is_some() {
        let tag = Tag::Generic(
            TagKind::from("since"),
            vec![filter.since.unwrap().to_string()],
        );
        tags.push(tag);
    }
    if filter.until.is_some() {
        let tag = Tag::Generic(
            TagKind::from("until"),
            vec![filter.until.unwrap().to_string()],
        );
        tags.push(tag);
    }
    if filter.generic_tags.len() > 0 {
        for t in filter.generic_tags {
            let tag = Tag::Generic(
                TagKind::from(format!("#{:#?}", t.0.to_string().to_lowercase())),
                t.1.iter().map(|v| v.to_string()).collect(),
            );
            tags.push(tag);
        }
    }

    if description.is_some() {
        let tag = Tag::Generic(TagKind::from("description"), vec![description.unwrap()]);
        tags.push(tag);
    }

    if id.is_some() {
        let tag = Tag::Generic(TagKind::D, vec![id.unwrap()]);
        tags.push(tag);
    }

    let tag = Tag::Generic(
        TagKind::from("noscript"),
        vec!["wasm:msg:filter".to_string()],
    );
    tags.push(tag);

    return tags;
}
