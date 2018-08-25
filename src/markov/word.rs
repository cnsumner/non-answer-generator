use rand;
use rand::Rng;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Word {
    word: String,
    successors: HashMap<String, i32>,
}

impl Word {
    pub fn word(&self) -> &str {
        self.word.as_str()
    }

    pub fn get_successor(&self) -> Option<String> {
        let mut rng = rand::thread_rng();

        if self.successors.is_empty() {
            return None;
        }

        let mut random_weight: i32 =
            rng.gen_range(0, self.successors.values().fold(0, |acc, &x| acc + x)) + 1;

        for s in self.successors.keys() {
            random_weight -= self.successors[s];

            if random_weight <= 0 {
                return Some(s.to_string());
            }
        }

        return None;
    }

    pub fn add_successor(&mut self, word: &String) {
        match self.successors.contains_key(word) {
            true => {
                *self.successors.get_mut(word).unwrap() += 1;
            },
            false => {
                self.successors.insert(word.to_string(), 1);
            }
        };
    }

    pub fn new(word: &String) -> Word {
        Word {
            word: word.to_string(),
            successors: HashMap::new(),
        }
    }
}
