# Repunit
<table>
<thead>
<tr>
<th style="text-align:center">
<a href="README_ja.md">日本語</a>
</th>
<th style="text-align:center">
<a href="README.md">English</a>
</th>
</tr>
</thead>
</table>

**日本語版は[こちら](./README_ja.md)にあります**

Repunit Generation Library

## Installation

    $ cargo install repunit

If you use this in your application, add these lines in `Cargo.toml`:

```rust
[dependencies]
repunit
```

## Usage

### First step
```rust
extern crate repunit;
```

### Convert from number of digits in Repunit to Repunit
```rust
repunit::convert(digits)
```
### Restore from Repunit to number of digits in Repunit
```rust
repunit::restore(Repunit)
```
### example
```rust
extern crate repunit;
fn main() {
    println!("{}",repunit::convert(repunit::restore(111111)));
}

// => 111111
```
```rust
extern crate repunit;
fn main() {
    println!("{}", 3 * repunit::convert(4));
}

// => 3333
```

## Development

To install this library onto your local machine, run `cargo install --path .`. To release a new version, update the version number in `Cargo.toml`, and then run `cargo build --release`, which will create a git tag for the version, push git commits and the created tag.

## Contributing

Bug reports and pull requests are welcome on GitHub at [https://github.com/NSK-1010/rust-repunit](https://github.com/NSK-1010/rust-repunit).

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).