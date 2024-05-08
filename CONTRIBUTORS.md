# Contributions

The following describes how to run Spritzer locally

## Pre-reqs for building

1. Install Rust via rustup [Install Rust](https://www.rust-lang.org/tools/install)
1. Install [Visual Studio Code](https://code.visualstudio.com/) and the Rust Analyzer extension

> NOTE: This allows building/debugging (`cargo build`) from the `library` project. Additional pre-reqs are needed to build the whole workspace. See below.

### Building `docs` project

1. Ensure a Visual C++ Redistributable (Windows) or GCC (Linux) is installed
1. Run `rustup target add wasm32-unknown-unknown` to install the web assembly compiler
1. Run `cargo install wasm-bindgen-cli wasm-pack` (Web Assembly toolchains)
1. Install the LTS version of [NodeJs](https://nodejs.org/en)
1. Run `npm i` from the main workspace.
1. Run `npm run docs:build`

### Building `python` project

1. Install a stable version of [Python 3](https://www.python.org/downloads/)
1. Install the Pylance Visual Studio Code extension
1. (optional) install a virtual Python environment
1. Run `cargo install maturin` (CPython toolchain)
1. Run `maturin build` or `maturin develop` from the `python` directory

### Building `assembly` files

> IMPORTANT: This is only necessary when modifying `.asm` files. The instructions are written for a Windows environent, but a Pull Request to add instructions for Linux/MacOS are welcome

1. Download a recent copy of `asar.exe` [ASAR](https://github.com/RPGHacker/asar) and place it in the ./assembly directory
1. Remove the first line from the generated .rs file (i.e. spritzer_base.rs)
1. Run `cargo build` to see the output of the operation.

   Rust Analyzer will also rebuild the file automatically. If an error condition occurs, pause the Rust Analyzer Studio in Visual Studio Code and then run `cargo build`

## Testing

- Run `cargo test` from the workspace or from a specific project folder. Tests that require real game binary data are ignored unless directly specified.
- Click "Run Test" in Visual Studio Code on the desired test.

## Debugging the library

> NOTE: no .sfc, .smc, or binary game data may be checked in

1. Place a sample .sfc in library/.testdata. The default name for test files is data.sfc
1. Go into docs/debug.rs in Visual Studio and find the main function.
1. Click on Run or Debug on the main function. This may take a moment to show up if Rust Analyzer is analyzing
1. The output will added to docs/.build/output.sfc

## Debugging the website/docs

- Run `npm run docs:dev` to run the web version

> NOTE: There is currently a race condition in wasm webpack that causes this command to throw a "rust error", but there is no error. The workaround is to comment out one of the minor config parameters in docs/.vuepress/config.js, save, uncomment it, and save again. This does not affect the ability to build and deploy the docs website. Rollup is under consideration as a result of this annoyance.

- Alternatively, `npm run docs:build` and launch a webserver on from that directory with a base url of `/spritzer/`

## Rebuilding Assembly

- Have a copy of asar.exe in the assembly directory.
- Remove the hash comment from the generated file and run `cargo build`.
- Rust Analyzer will also automatically regenerate the file when that line is removed.

## Architecture

This is a workspace project for Rust. At any time, running `cargo build` will compile all compilable targets. There are a few different projects:

- **assembly** - Contains direct Assembly code needed to reprogram parts of the game. This automatically generates Rust code that contains the bytes generated from the code. However, this does not use a copy of the game and instead diffs off an "empty" file of 4MB. As a result, it is not possible to `org` 2 addresses that are not at least 2 bytes away from each other. The current marker is 0xFF 0xFF to detect "empty".
- **docs** - Contains documentation and a web version of Spritzer. Also contains a program for debugging.
- **library** - This contains the main logic of the randomizer and contains models for working with objects in the game, reader/writers for performing conversions, and logic to perform transformations on dungeons, sprites, overworld, etc. This is the majority of code in the project.
- **python** - This contains bindings to convert Rust code to python, so this project can be run from python frontends such as Archipelago
