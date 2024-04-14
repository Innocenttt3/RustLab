# Rust Programming Repository

This repository is dedicated to storing all the tasks completed during the Rust programming classes, as well as solutions to quizzes and exams. It is continuously evolving to reflect progress made during the classes. This course is part of the curriculum for the 4th semester of Computer Science studies at UMCS.

# Basics

RustUP is a script system that installs the Rust environment and automatically updates it.

## Rust Lang Installation

To install Rust, visit the [Rust Lang website](https://www.rust-lang.org/) and follow the instructions.

## Usage

1. To create a new project, use: `cargo new <project_name>`

2. To build the project, use: `cargo build`


3. To run the project, use: `cargo run`


### Additional Commands

- After running `cargo build` or `cargo run`, two new files may appear:
- `cargo.lock`: Do not modify manually.
- `target`: This directory is automatically added to .gitignore as it contains compilation files and debris.

- To run a specific binary file, use: `cargo run --bin <specific_file_name>`


- To clean up the files, equivalent to shift+command+L in Rust Rover Convert, use: `cargo fmt`
