# alua

A Rust library for generating [luals](https://luals.github.io/) annotations from Rust types.

# Example

```rs
use alua::*;

#[derive(ClassAnnotation)]
#[alua(fields = [
    "method fun(self: Example, message: string) - Send a message",
])]
struct Example {
    /// test
    #[allow(unused)]
    field: u32,
}

fn main() {
    print!("{}", Example::class_annotation());
}
```

This produces the following output:

```lua
--- @class Example
--- @field field integer - test
--- @field method fun(self: Example, message: string) - Send a message
```
