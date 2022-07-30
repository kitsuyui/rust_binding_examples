# amazing-calc

This repo is intended to show how to build rust-python binding packages.

```python
from amazing_calc import my_calc
my_calc(1, 2, 3)  # => '9'
```

`my_calc` is implememented by Rust (with PyO3). 
However it works as same as following python code:

```python
def my_calc(a: int, b: int, c: int) -> str:
    return str((a + b) * c)
```
