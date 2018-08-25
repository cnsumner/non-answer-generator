extern crate config;
extern crate rand;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
// #[macro_use]
extern crate serenity;
extern crate typemap;
extern crate openssl_probe;

use config::{Config, File};
use rand::Rng;
use serenity::client::Client;
// use serenity::framework::standard::StandardFramework;
use serenity::model::channel::Message;
use serenity::prelude::EventHandler;
use serenity::prelude::*;

mod markov;
use markov::Markov;

struct Handler;

impl EventHandler for Handler {
	fn message(&self, ctx: Context, msg: Message) {
		if msg.author.bot {
			return;
		}

		let mut data = ctx.data.lock();
		let m: &mut Markov = data.get_mut::<Markov>().unwrap();

		let mut response = String::new();

		if msg.content.starts_with("~") {
			let args: Vec<_> = msg.content.split(" ").collect();
			match args[0] {
				"~random" => {
					response =
						m.generate_sentence(20, [m.get_random_word().word()].to_vec().to_owned());
				}
				"~gen" => {
					let count = match args[1].to_string().parse::<i32>() {
						Ok(i) => i,
						Err(_e) => 0,
					};
					let seed_words = match args.get(2..args.len()) {
						Some(slice) => slice.to_vec(),
						None => Vec::new(),
					};
					if count > 0 && count <= 250 && seed_words.len() > 0 {
						response = m.generate_sentence(count, seed_words);
					} else {
						response = "You broke my bot code. Try it like this: `~gen [number from 1 to 250] [one or more words]`".to_string();
					}
				}
				"~query" => {
					response = "Command not implemented yet.".to_string();
				}
				"~info" => {
					response = format!("I am growing stronger. I know {} words.", m.known_words());
				}
				_ => {
					response = "Invalid command.".to_string();
				}
			};
		} else {
			let mut rng = rand::thread_rng();

			if rng.gen_range(1, 21) == 20 {
				println!("Rolled a 20! Sending a random message...");
				response = m.generate_sentence(
					rng.gen_range(1, 6),
					match msg.content.len() {
						0 => [m.get_random_word().word()].to_vec().to_owned(),
						_ => [*msg
							.content
							.split(" ")
							.collect::<Vec<&str>>()
							.first()
							.unwrap()]
							.to_vec()
							.to_owned(),
					},
				);
			}

			if !msg.content.contains("http://") {
				m.train(&msg.content);
				m.save();

				println!("Trained off of sample [\n{}\n]", msg.content);
			}
		}

		if response.len() > 0 {
			match msg.channel_id.say(response) {
				Ok(_) => (),
				Err(e) => println!("Error sending message: {:?}", e),
			}
		}
	}
}

fn main() {
	openssl_probe::init_ssl_cert_env_vars();
	
	let mut config = Config::default();
	config
		.merge(File::with_name("config"))
		.expect("Config file missing!");

	let m = Markov::new("kb.json".to_string());

	let mut client = match Client::new(
		&config
			.get_str("DISCORD_TOKEN")
			.expect("Config file does not contain key DISCORD_TOKEN"),
		Handler,
	) {
		Ok(c) => c,
		Err(e) => panic!("Error creating client: {:?}", e),
	};

	{
		let mut data = client.data.lock();
		data.insert::<Markov>(m);
	}

	if let Err(why) = client.start() {
		println!("Client error: {:?}", why);
	}
}
