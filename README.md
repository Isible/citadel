# citadel

## Introduction

An experimental compiler backend

## Setup

1. Install [Rust](https://www.rust-lang.org/) and [Python](https://www.python.org/) minimum py-version: 3.12

2. [Clone the repository](https://docs.github.com/en/repositories/creating-and-managing-repositories/cloning-a-repository)

3. Try building the repository using `cargo build` in the `citadel` root folder

4. [Optional] Run the tests using `python _scripts/tests.py` and then follow the instructions

## Project-structure

- [Api](api) - The api thats exposed to the developer. Has lots of qol features and bundles all 3 ends so the developer doesn't need to interact with these directly.

- [Frontend](frontend) - Frontend for citadel that generates IR from your source code

- [Middleend](middleend) - Middleend for citadel that optimizes source code and generates multiple more low-level IRs from this depending on the optimization level

- [Backend](backend) - Backend that compiles IR to machine-code

- [Test-lang](test-lang) - Language for testing citadel

- [Fungus](fungus) - Experimental rust compiler that uses Citadel. Will be moved to its own repository once citadel is done
