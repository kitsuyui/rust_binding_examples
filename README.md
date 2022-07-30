# rust_binding_examples

This repo is intended to show how to build rust-python binding packages.

## Example rust module

`amazing-calc` provides very simple function as following:

```rust
pub fn my_calc(a: i64, b: i64, c: i64) -> String {
    ((a + b) * c).to_string()
}
```

```rust
my_calc(1, 2, 3)  // => "9"
```

## Build python package

```sh
cd python
pipenv run build
```

# LICENSE

The 3-Clause BSD License. See also LICENSE file.