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

    // Send test event
    client.publish_text_note("My first text note from Nostr SDK!", []).await?;
    client.publish_text_note("what is the best podcast to subscribe to? This Week in Tech is a great podcast.  It's hosted by Leo Laport (of TechTV fame) and features several guests each week, usually other ex-TechTV people and John Devorak.  It's basically a discussion of tech news stories from the week.  It's been the top podcast for some time now, probably because of the well known, intelligent hosts as well as the professional production quality.\n\nBrowsing the podcasts available on iTunes can be a good way to easily sample some podcasts on topics you're interested in.", []).await?;
    client.publish_text_note("motor vehicle agencys in central newjersey?rt 35 by the monmouth mall, across the street form the jeep dealer ship", []).await?;
    client.publish_text_note("I am trying to mount a dual boot(linux/windows)\ndell laptop internal ide (ibm) 20 gig hard disk\non my dell precision desktop (wind2k) using a ide to usb cable.(sabrent).\n\nI can see my drive in the device manager under disk drives, it kwon exactly my disk id (IC25T048 ATDA05-0 USB Device. but when it wont show in the disk management window.\n\nHow can I mount this drive on my pc???", []).await?;

    Ok(())
}
