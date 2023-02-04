Migrate to https://github.com/kitsuyui/rust-playground

# rust_binding_examples

[![codecov](https://codecov.io/gh/kitsuyui/rust_binding_examples/branch/main/graph/badge.svg?token=MVD7SO576O)](https://codecov.io/gh/kitsuyui/rust_binding_examples)

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

## Python package

See also [/python/README.md](/python/README.md)

### Install

Warning: Currently this package is only distributed on TestPyPI.

```sh
pip install -i https://test.pypi.org/simple/ amazing-calc
```

```python
from amazing_calc import my_calc
my_calc(1, 2, 3)  # => '9'
```

### Build

```sh
cd python
pipenv run build
```

# LICENSE

The 3-Clause BSD License. See also LICENSE file.
