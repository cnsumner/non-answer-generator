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

How do I add it to my server?
------------

_coming soon(tm)_

Can you add X feature?
------------

Maybe. If I have time and think it's worth while.

Better yet, learn rust (if you don't already know it), add the feature yourself and submit a pull request so I can add your work to the repository.  Contributions are welcome and encouraged.
