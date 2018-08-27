Non-Answer Generator: a Discord Bot
===================================

_Disclaimer: this is a partially completed Rust port of a bot I wrote in Dart and is not fully featured, nor bug free._

**TL;DR: this bot sits in your discord server and learns how to talk like your users**

What
----
Machine learning at it's finest. This is it boys.

This is a Discord chat bot built on top of a custom [Markov Chain](https://en.wikipedia.org/wiki/Markov_chain) implementation.  It listens to nearly every message sent in your discord server and uses those messages as sample text to train a statistical model.  

As it learns, it will be able to generate messages that resemble things that your discord server users might say. It gets better (or worse, depending on your perspective) over time.

It will occasionally respond to messages with often nonsensical, but sometimes eerily topical responses.

Why
----

Why not?

Usage
-----

The bot will learn and also respond on it's own, no need to @ it or anything like that.

By default the bot has a 1 in 20 chance of saying something every time a message is sent on the server (this will be configurable later).

The following command can also be used:
- `~random` generates a random messages
- `~gen [n] [words]` where `[n]` should be a number (1 to 250) and `[words]` is a list of words (space separated) to start with
- `~info` displays the number of words known by the bot

How do I add it to my server?
------------

### Installation (follow instructions for your operating system below, then continue to configuration instructions)
#### Linux

1. Download [the latest build](https://gitlab.com/cnsumner/non-answer-generator/-/jobs/artifacts/master/download?job=build-linux)
2. Upzip `artifacts.zip` somewhere you want to keep it

#### Windows or some other system

1. Go [here](https://www.rust-lang.org/en-US/install.html) and follow the instructions to install Rust on your system
2. Download this repository and unzip it somewhere
3. Open a command prompt or terminal in the folder you just unzipped
4. Run the command `cargo build --release` and let it finish (it will take a few minutes)
5. Open up the new folder that got created: `./target/release/`
6. Take the binary file (`non-answer-generator.exe` on Windows, or `non-answer-generator` on Linux) and toss it in the `example` folder back in the first folder you unzipped

### Configuration

1. Edit `config.json` and add your discord bot token to it (if you don't know where to get this, google it)
2. Run the binary file/executable
3. Do a Google search to figure out how to invite the bot to your server
4. Send some messages and enjoy the chaos

Can you add X feature?
------------

Maybe. If I have time and think it's worth while.

Better yet, learn rust (if you don't already know it), add the feature yourself and submit a pull request so I can add your work to the repository.  Contributions are welcome and encouraged.
