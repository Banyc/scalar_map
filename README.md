# `scalar_map`

`map` for scalar types.

```rust
let num: Option<i32> = Some(42);
assert_eq!(num.map(|x| 42 - x), Some(0));

let num: i32 = 42;
assert_eq!(num.map(|x| 42 - x), 0);

let num: Option<i32> = Some(42);
assert_eq!(num.and_then(Option::Some), Some(0));

let num: i32 = 42;
assert_eq!(num.and_then(Option::Some), Some(0));
```

## What does it solve

- You can still keep `map` and `and_then` intact even if `Option` is refactored out.
- You want

  ```rust
  x.map(Mutex::new).map(Arc::new)
  ```

  ...instead of

  ```rust
  Arc::new(Mutex::new(x))
  ```

## Custom struct

### Deriving

```bash
# Run this to add the `derive` feature
cargo add scalar_map --features derive
```

```rust
#[derive(Debug, PartialEq, ScalarMap)]
struct MyNum(i32);

let num = MyNum(42);
assert_eq!(num.map(|x| 42 - x.0).map(MyNum), MyNum(0));
```

### Without deriving

```rust
#[derive(Debug, PartialEq)]
struct MyNum(i32);
impl ScalarMapExt for MyNum {}

let num = MyNum(42);
assert_eq!(num.map(|x| 42 - x.0).map(MyNum), MyNum(0));
```
