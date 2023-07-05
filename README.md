A simple attribute that makes all fields public on a struct.

Usage:
```rust
#[pub_fields]
pub struct MyStruct {
    a: usize,
    b: usize,
    c: usize,
}
```
=>
```rust
pub struct MyStruct {
    pub a: usize,
    pub b: usize,
    pub c: usize,
}
```
