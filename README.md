# Citadel

## Introduction

An experimental compiler backend with a similar approach as llvm but trying to be more modular, simpler and overall more intuitive.

Citadel: A bastion to protect from the dragon :p

## Design

Citadel is designed to be self-hosted, meaning you do not need to install anything besides the rust crates. Infact, Citadel itself is only an api that allows developers to create optimizer, backends and of course their compiler.

## Mission

The mission of this project is to empower more people to try building compilers and for making rust a better option for building them. Let the compiler wars begin :p

## Setup

1. Install [Rust](https://www.rust-lang.org/) and **(Optional)** [Python](https://www.python.org/) minimum py-version: 3.12

2. [Clone the repository](https://docs.github.com/en/repositories/creating-and-managing-repositories/cloning-a-repository)

3. Try building the repository using `cargo build` in the `citadel` root folder

4. **(Optional)** Run the tests using `python scripts/tests.py all`

## Project-structure

- [Api](api) - The api thats exposed to the developer. Has lots of qol features and bundles all 3 ends so the developer doesn't need to interact with these directly.

- [Frontend](frontend) - Frontend for citadel that helps with generating IR from your source code

- [Middleend](middleend) - Middleend for citadel that optimizes source code and generates multiple more low-level IRs from this depending on the optimization level

- [Backend](backend) - Backend that compiles IR to machine-code

- [Test-lang](test-lang) - Language for testing citadel

- [Ciri](ciri) - an interpreter for citdel intermediary representation

- [IrParser](irparser) - a parser for tokenizing and parsing IR into valid IR statements as specified in the frontend. This is also used for ciri

## Tooling

- [Test-lang](test-lang) For testing, experimenting and providing an example on how to build a compiler with citadel

- [Ciri](ciri) - an interpreter for citadel IR to debug IR code

- [Citadel-fmt] - a formatter for citadel IR and a toolkit for building formatters

- [Citadel-lsp] - a lsp for citadel IR and a toolkit for building lsps
