# Hangeul-id-rs

An unique shorter id library for rust using Hangeul inspired by Hashids

## Features

- Generate Hangeul id from decimal integer

## Usage

```toml
[dependencies]
hangeul-id = "0.1.0"
```

```rust
use hangeul_id::HangeulId;

fn main() {
    let hangeulid = HangeulId::new();
    let id = hangeulid.int_to_hangeul_id(12345678);
    println!("{}", id) // print hangeul id matching 12345678
}
```