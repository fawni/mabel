# enolib

A feature-complete, pure rust parser for Eno.

Note on packaging: Not yet released as a crate, but can be pulled in via git regardless.

Note on stability: The parsing itself is quite stable, the API is currently still in flux though.

## Getting started

Add the library to the dependencies in your `Cargo.toml`:

```toml
[dependencies]
...
enolib = { git = "https://codeberg.org/simonrepp/enolib-rs" }
```

Some example code to get you started:

```rust
use enolib;

fn main() {
    let result = my_parse("greeting: hello");
    
    dbg!(result); // Ok("hello")
}

fn my_parse(input: &str) -> Result<String, enolib::Error> {
    enolib::parse(input)?
      .field("greeting")?
      .required_value()
}
```

Until there is more documentation, I recommend you clone the repository and in
its root run this to generate and access the documentation in your browser:

```sh
cargo doc --no-deps --open
```

From there go to the documentation for `parse` and traverse the documentation
for what it returns downwards, until you are at the bottom of the tree, where
you have everything you need to parse any eno document!
