# Contributions

The following describes how to run Spritzer locally

## Pre-reqs for Building

1. Install Rust via rustup [Install Rust](https://www.rust-lang.org/tools/install)
2. Install [Visual Studio Code](https://code.visualstudio.com/) and the Rust Analyzer extension
3. Run `rustup target add wasm32-unknown-unknown` to install the web assembly compiler
4. Run `cargo install wasm-bindgen-cli wasm-pack maturin` (various toolchains)
5. Install the LTS version of [NodeJs](https://nodejs.org/en)
6. Run `npm i` from this directory

> NOTE:  If working with assembly, download a recent copy of asar.exe from [Github](https://github.com/RPGHacker/asar) and place it at the root of the assembly project. This is only necessary if changing the .asm files

## Running

- Run the task, "Python: Main UI" from the Visual Studio Code debug menu.
- Run `npm run docs:dev` to run the web version

## Rebuilding Assembly

 - Have a copy of asar.exe in the assembly directory. 
 - Remove the hash comment from the generated file and run `cargo build`. 
 - Rust Analyzer will also automatically regenerate the file when that line is removed.

## Architecture

This is a workspace project for Rust. At any time, running `cargo build` will compile all compilable targets. There are a few different projects:

- **assembly** - Contains direct Assembly code needed to reprogram parts of the game. This automatically generates Rust code that contains the bytes generated from the code. However, this does not use a copy of the game and instead diffs off an "empty" file of 4MB. As a result, it is not possible to `org` 2 addresses that are not at least 2 bytes away from each other. The current marker is 0xFF 0xFF to detect "empty".
- **docs** - Contains documentation and a web version of Spritzer.
- **library** - This contains the main logic of the randomizer and contains models for working with objects in the game, reader/writers for performing conversions, and logic to perform transformations on dungeons, sprites, overworld, etc. This is the majority of code in the project.
- **python** - This contains bindings to convert Rust code to python, so this project can be run from python frontends such as Archipelago
- **ui** - A python UI for running the mod. This is largely used for local testing and will be sunsetted.
