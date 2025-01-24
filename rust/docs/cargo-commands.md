# Cargo Commands

This is a page summarizing frequently used
commands.

[official documentation](https://doc.rust-lang.org/cargo/commands/index.html)

<details>
  <summary><b>Table of Contents</b></summary>

- [General Commands](#general-commands)
- [Build Commands](#build-commands)
    - [cargo run](#cargo-run)
    - [cargo doc](#cargo-doc)
    - [cargo add](#cargo-add)

</details>

---

## General Commands

## Build Commands

### cargo run

Run the current package

```shell
cargo run
# Do not print cargo log messages.
cargo run -q 
```

### cargo doc

```shell
# Generate Documentation
cargo doc
# Open the Documentation
cargo doc --open
```

### cargo add

```shell
cargo add {dependency name}
```


