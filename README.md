<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->

# rustpg
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-informational)](COPYRIGHT.md)
![rust-latest](https://img.shields.io/badge/rust-stable-orange)
![bevy-0.9](https://img.shields.io/badge/bevy-0.9-lightgray)

 **rustpg** is a springboard for developing and testing our RPG-focused Bevy plugins.

## Features
* ... Not much yet!

## Getting started

This section outlines how to get this project up and running locally.

1. Install the latest stable version of Rust, in accordance with [the Rust book](https://doc.rust-lang.org/book/ch01-01-installation.html).
2. Clone this repository.
3. Open your local clone, preferably using [vscode](https://code.visualstudio.com). 
    - Remember to check out the extension suggestions!
4. Make sure your fresh clone builds and runs, by running an example, like `cargo run --example nychtemeron`.

That's it! You're ready to start contributing! 💪

## Contributing
To contribute to this project, create a branch in accordance with our branching strategy and start working on an issue. When you're done implementing, create a PR targetting the `develop` branch.

### Branching strategy
This repository uses feature branching. When working on an issue, create a branch from `develop` using the naming convention 
* `feature/#-title` for enhancements
*  `fix/#-title` for bugs.

Example: *You assign yourself to the [dummy issue #1](https://github.com/Nilsiker/rustpg/issues/1), which is labelled as an enhancement. The branch would then be named `feature/1-dummy-issue`.*

### Tips
Before creating your PR, make sure that
1. You've formatted your code using `cargo fmt --all`.
2. No errors or warnings are returned when using `cargo clippy -- -D warnings`
3. All tests pass when running `cargo test`

Using [bacon](https://github.com/Canop/bacon), you can easily run `clippy` and other cargo commands in the background, while you are developing. I highly recommend you try it out!

## Licensing
This project is licensed using a dual MIT/Apache license in accordance with [the COPYRIGHT.md](COPYRIGHT.md).

The project comes bundled with the [Cascadia Code font](https://github.com/microsoft/cascadia-code), which goes under the [SIL Open Font License](https://github.com/microsoft/cascadia-code/blob/main/LICENSE)