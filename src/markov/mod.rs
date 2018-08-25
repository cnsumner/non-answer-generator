mod word;

use markov::word::Word;
use rand;
use rand::Rng;
use regex::Regex;
use serde_json;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;
use typemap::Key;

pub struct Markov {
    path: String,
    db: HashMap<String, Word>,
}

impl Key for Markov {
    type Value = Markov;
}

impl Markov {
    ///Returns a random entry from the database of known words
    pub fn get_random_word(&self) -> &Word {
        let mut rng = rand::thread_rng();
        let random_key: String = match self.db.keys().nth(rng.gen_range(0, self.db.len())) {
            Some(key) => key.to_string(),
            None => {
                panic!(
                    "Tried to access out of bounds element in HashMap. This should never happen."
                );
            }
        };

        self.db.get(&random_key).unwrap()
    }

    ///Generates a string of at least `count` words, beginning with `seed_word`
    pub fn generate_sentence(&self, count: i32, seed_words: Vec<&str>) -> String {
        let mut seed_word = seed_words.last().unwrap().to_string();
        let mut sentence = String::from(seed_words.join(" "));
        let mut word_count = 1;
        while word_count <= count || "for|and|that|the|on|by|in|can|a|with|so|i|of|to|you|but|be|he|she|cuz|very|oh".contains(seed_word.as_str()) {
            sentence.push(' ');
            if !(self.db.contains_key(&seed_word)) || seed_word == "" {
                seed_word = self.get_random_word().word().to_string();
            } else {
                seed_word = match self.db.get(&seed_word).unwrap().get_successor() {
                    Some(word) => word.to_string(),
                    None => self.get_random_word().word().to_string(),
                };
            }
            sentence.push_str(&seed_word);
            word_count += 1;

            if sentence.len() > 1900 {
                break;
            }
        }

        sentence
    }

    pub fn train(&mut self, sample: &String) {
        let sample = &sample.to_lowercase();
        let tokens: Vec<&str> = Regex::new("[,.?!`()] ?|'s ?| +")
            .unwrap()
            .split(sample)
            .collect();

        for i in 0..tokens.len() {
            let token = tokens[i].to_string();
            let next_word = if i < tokens.len() - 1 {
                Some(tokens[i + 1].to_string())
            } else {
                None
            };
            if !(self.db.contains_key(&token)) {
                self.db.insert(token.to_string(), Word::new(&token));
            }

            match next_word {
                Some(word) => self.db.get_mut(&token).unwrap().add_successor(&word),
                _ => (),
            };
        }
    }

    ///Returns an immutable reference to the database HashMap
    // pub fn db(&self) -> &HashMap<String, Word> {
    //     &self.db
    // }

    ///Returns the number of known words
    pub fn known_words(&self) -> usize {
        self.db.len()
    }

    ///Serializes and saves known words to disk
    pub fn save(&self) {
        let mut file = <OpenOptions>::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(self.path.clone())
            .unwrap();

        let raw_json = match serde_json::to_string(&self.db) {
            Ok(json) => json,
            Err(e) => panic!(
                "Error serializing JSON, this should really never happen: {:?}",
                e
            ),
        };

        file.write_all(raw_json.as_bytes())
            .expect("Error writing to file... This should probably never happen.");
    }

    pub fn new(path: String) -> Markov {
        let mut file = <OpenOptions>::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path.clone())
            .unwrap();
        let mut raw_json = String::new();

        file.read_to_string(&mut raw_json)
            .expect("Failed to read db... This should never happen."); //TODO: might want to properly handle this even though "it will never happen"

        Markov {
            path: path,
            db: match serde_json::from_str(&raw_json) {
                Ok(hash_map) => hash_map,
                Err(e) => {
                    println!("Empty file (normal for first run) or error deserializing JSON (see error): {:?}", e);
                    HashMap::new()
                }
            },
        }
    }
}
