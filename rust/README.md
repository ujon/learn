# Learn Rust

<details>
  <summary><b>Table of Contents</b></summary>

- [Installation](#installation)
    - [MacOS](#macos)
- [What is](#what-is)
    - [rustup](#rustup)
    - [Cargo](#cargo)
    - [Crate](#crate)
- [How to](#how-to)
    - [Show the active and installed toolchains or profiles](#show-the-active-and-installed-toolchains-or-profiles)
    - [Create project](#create-project)
    - [Run project](#run-project)
    - [Add dependency(crate)](#add-dependencycrate)
- [Links](#links)

</details>

---

## Installation

### MacOS

[Official](https://rustup.rs/)

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Homebrew

```shell
# Don’t do this
brew install rust
# Do this
brew install rustup
# Once you have installed rustup using homebrew you can run rustup-init and select the option 1
rustup-init
```

## What is

### rustup

[rustup](https://rustup.rs/) is a command-line tool used to manage Rust programming language installations, versions,
and associated tools on your system. It is the recommended way to install and maintain Rust.

### Cargo

[official documentation](https://doc.rust-lang.org/cargo/index.html)

`cargo` is Rust's official package manager and build system. It simplifies the process of managing Rust projects, their
dependencies, building, and running code. It comes bundled with the Rust

### Crate

[crates.io](https://crates.io/)

`crate` == `package`

`crate` is the fundamental compilation unit and the primary way to organize and share code. Think of a crate as a
package or library that can be compiled into executable binaries or reusable libraries.

### Ownership

### Borrowing

### Lifetimes

## How to

### Show the active and installed toolchains or profiles

```shell
rustup show
```

### Create project

```shell
cargo new {projectname}
```

### Run project

```shell
cargo run
```

### Add dependency(crate)

```shell
cargo add {dependency name}
# example
cargo add rand
```

## Links

- [Rust std documentation (official)](https://doc.rust-lang.org/stable/std/index.html)
- [Cargo commands](docs/cargo-commands.md)
- [Rust syntax](docs/rust-syntax.md)