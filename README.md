# alua

A Rust library for generating [luals](https://luals.github.io/) annotations from Rust types.

# Example

```rs
use alua::*;

#[derive(ClassAnnotation)]
#[alua(fields = [
    "find_code_references fun(code string) CodeReferenceWithDocument[]",
    "find_discrete_values fun(unique_id string) DiscreteValue[]",
    "find_medications fun(external_id string) Medication[]",
    "find_documents fun(document_id string) CACDocument[]",
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
--- @field find_code_references fun(code string) CodeReferenceWithDocument[]
--- @field find_discrete_values fun(unique_id string) DiscreteValue[]
--- @field find_medications fun(external_id string) Medication[]
--- @field find_documents fun(document_id string) CACDocument[]
```
