# Rust Syntax

## Implicit Return

Below are two functions that are exactly the same:

```rust
fn is_even(num: i32) -> bool {
    return num % 2 == 0;
}

fn is_even(num: i32) -> bool {
    num % 2 == 0 // important! do not add ;(semicolon)
}
```