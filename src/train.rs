use crate::types::{Event, NOSCRIPT_KIND};
use csv;
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_writer};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::time::{SystemTime, UNIX_EPOCH};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Default, Serialize, Deserialize)]
struct Counter {
    ham: u32,
    spam: u32,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Train {
    token_table: HashMap<String, Counter>,
}

impl Train {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn train(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        let mut rdr = csv::Reader::from_path(path)?;
        for result in rdr.records() {
            match result {
                Ok(record) => {
                    let msg = record.get(1).unwrap();
                    if msg.eq("4") {
                        // computer & science class
                        let train_msg = record.get(2).unwrap();
                        self.train_spam(train_msg);

                        let s1 = record.get(2).unwrap();
                        self.train_spam(s1);

                        let s2 = record.get(3).unwrap();
                        self.train_spam(s2);
                    } else {
                        let train_msg = record.get(2).unwrap();
                        self.train_ham(train_msg);

                        let s1 = record.get(2).unwrap();
                        self.train_ham(s1);

                        let s2 = record.get(3).unwrap();
                        self.train_ham(s2);
                    }
                }
                Err(err) => println!("{:#?}", err),
            }
        }

        println!("trained, total tokens: {:#?}", self.token_table.len());
        Ok(())
    }

    pub fn save_vecs(&self) -> Result<(), io::Error> {
        let mut spams: Vec<u32> = vec![];
        let mut hams: Vec<u32> = vec![];
        let mut words: Vec<String> = vec![];
        for (key, value) in self.token_table.iter() {
            spams.push(value.spam);
            hams.push(value.ham);
            words.push(key.to_string());
        }

        let spam_file = File::create("spam-vec.txt")?;
        let ham_file = File::create("ham-vec.txt")?;
        let word_file = File::create("tokens.txt")?;

        to_writer(spam_file, &spams)?;
        to_writer(ham_file, &hams)?;
        to_writer(word_file, &words)?;

        Ok(())
    }

    pub fn from_local_vecs_to_event() -> Result<Event, io::Error> {
        let spams = read_vec("spam-vec.txt").unwrap();
        let hams = read_vec("ham-vec.txt").unwrap();
        let mut token_vec = read_tokens("tokens.txt").unwrap();

        let mut spams: Vec<String> = spams.iter().map(|v|v.to_string()).collect();
        spams.insert(0, "spams".to_string());
        let mut hams: Vec<String> = hams.iter().map(|v|v.to_string()).collect();
        hams.insert(0, "hams".to_string());
        token_vec.insert(0, "tokens".to_string());

        let file: File = File::create("algo_event.json")?;
        let duration_since_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let timestamp_seconds = duration_since_epoch.as_secs() as i64;

        let event: Event = Event {
            content: "".to_string(),
            created_at: timestamp_seconds,
            id: "".to_string(),
            kind: NOSCRIPT_KIND,
            pubkey: "".to_string(),
            sig: "".to_string(),
            tags: vec![spams, hams, token_vec],
          };

        to_writer(file, &event)?;
        Ok(event)
    }

    pub fn save_event(&self) -> Result<Event, io::Error> {
        let mut spams: Vec<String> = vec!["spams".to_string()];
        let mut hams: Vec<String> = vec!["hams".to_string()];
        let mut words: Vec<String> = vec!["tokens".to_string()];

        for (key, value) in self.token_table.iter() {
            let spam = value.spam.to_string();
            let ham = value.ham.to_string();
            let token = key.to_string();
            spams.push(spam);
            hams.push(ham);
            words.push(token);
        }

        let file: File = File::create("algo_event.json")?;

        let duration_since_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let timestamp_seconds = duration_since_epoch.as_secs() as i64;

        let event: Event = Event {
            content: "".to_string(),
            created_at: timestamp_seconds,
            id: "".to_string(),
            kind: NOSCRIPT_KIND,
            pubkey: "".to_string(),
            sig: "".to_string(),
            tags: vec![spams, hams, words],
          };

        to_writer(file, &event)?;

        Ok(event)
    }

    pub fn from_local_vecs_to_args(
    ) -> Result<(HashMap<String, u32>, Vec<u32>, Vec<u32>), std::io::Error> {
        let spams = read_vec("spam-vec.txt").unwrap();
        let hams = read_vec("ham-vec.txt").unwrap();
        let token_vec = read_tokens("tokens.txt").unwrap();
        let mut tokens: HashMap<String, u32> = HashMap::new();

        for (index, token) in token_vec.iter().enumerate() {
            tokens.insert(token.to_string(), index as u32);
        }

        Ok((tokens, spams, hams))
    }

    /// Split `msg` into a list of words.
    fn load_word_list(msg: &str) -> Vec<String> {
        let word_list = msg.unicode_words().collect::<Vec<&str>>();
        word_list.iter().map(|word| word.to_string()).collect()
    }

    /// Train the classifier with a spam `msg`.
    pub fn train_spam(&mut self, msg: &str) {
        for word in Self::load_word_list(msg) {
            let counter = self.token_table.entry(word).or_default();
            counter.spam += 1;
        }
    }

    /// Train the classifier with a ham `msg`.
    pub fn train_ham(&mut self, msg: &str) {
        for word in Self::load_word_list(msg) {
            let counter = self.token_table.entry(word).or_default();
            counter.ham += 1;
        }
    }
}

pub fn read_vec(file: &str) -> Result<Vec<u32>, io::Error> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    let vec_data: Vec<u32> = from_reader(reader)?;
    Ok(vec_data)
}

pub fn read_tokens(file: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    let vec_data: Vec<String> = from_reader(reader)?;
    Ok(vec_data)
}
