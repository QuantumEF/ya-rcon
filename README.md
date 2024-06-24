# Yet Another RCON Library

This project is a rust implementation of the RCON protocol commonly used for game as defined here: [Source RCON Protocol](https://developer.valvesoftware.com/wiki/Source_RCON_Protocol)

## Feature overview

*   [x] Synchronous RCON API for sending commands and recieving a corresponding response.
*   [ ] Async functionality provided by "tokio" and "async_net" features (and maybe others?)
*   [ ] Contribution guide.
*   [ ] Docker containers for being able to test compatibility with various games.
*   [ ] Organization guide for addision of game specific abstractions gated with "features"

## Contents

*   [What is this?](#what-is-this)
*   [When should I use this?](#when-should-i-use-this)
*   [Getting started](#getting-started)
    *   [Requirements](#requirements)
    *   [Install](#install)
    *   [Usage](#usage)
*   [Here is where it's your turn](#here-is-where-its-your-turn)
*   [Don't forget anything](#dont-forget-anything)
    * [Used Technologies](#used-technologies)
    * [Testing](#testing)
    * [Logging](#logging)
*   [Contribute](#contribute)
*   [License](#license)
*   [Sources](#sources)
*   [Conclusion](#conclusion)

## What is this?

While looking at other RCON rust implementations, I noticed they were fairly old and unmaintained. I was having some weird dependency issues when trying to use them, so I decided to create my own with minimal dependency usage. I would *like* to try keep the base library free of dependencies other than the standard library (with the expection of optional dependencies used by extra features): though that may be an impossible task.

The idea is to keep the base library minimal and generic and have higher level features gated with the "features" flags provided by cargo.

Some of the other rcon libraries I came across were: [rust-rcon](https://github.com/panicbit/rust-rcon/tree/master), [rcon-rs](https://crates.io/crates/rcon-rs), and [rercon](https://github.com/ikkerens/rercon/tree/master).

## Why should I use this?

No good reason at the moment I guess. I am going to attempt to actually maintain the project. If you have *literally any* questions, comments, concerns, thoughts, issues, ideas, open a pull request. I want to actually dicuss things about this project with other people. 

## Getting Started

### Usage

Can't fill this section in quite yet until I do a bit more testing with the recieving of packets.

### Project ideas

A cli project that has gated features to do auto complete.

A web based console using leptos and axum (this is something I was originally planning on doing before running into those depencency issues.)

If either of those interest you, feel free to open an issue to discuss and bounce some ideas around. 

## Contribute

Pull requests are welcome. You should open an issue first to discuss what you would like to change. (We don't want multiple people trying to implement the same thing). Even if you don't really feel like implementing it, different perspectives are helpful when trying to create a library.

## License
[MIT](https://choosealicense.com/licenses/mit/)

# Another TODO (Mainly for myself since it is more haphazard and specific to communicating with myself.)
- tokio and async_net features.
- creates.io and documentation
    - Creation of docker container to test rcon with specific game (and document how to so other can do so.)
- HashMap for handling ["fragmented" packets](https://developer.valvesoftware.com/wiki/Source_RCON_Protocol#Multiple-packet_Responses)? 
- organization / investigation to determine game specific features for higher level abstractions.
    - contribution guide
- open isue for "std" AsyncWrite/Read trait in case it ends up in the standard library.
