This library provides functions for specializing on types dynamically via `Any`.
The basic idea is that it allows you to "cast" any `T` into any
other `U`, if `T` and `U` are actually the same  type.

This is exposed both as freestanding functions, as well as an extension trait
that can be imported.

# Example

```rust
use identity_cast::IdentityCast;

fn print_i32_specially<T: 'static>(v: T) {
    match v.into_same::<i32>() {
        Ok(v) => {
            println!("This is a `i32` with value {}", v);
        }
        Err(_) => {
            println!("This is some `T`");
        }
    }
}
```
