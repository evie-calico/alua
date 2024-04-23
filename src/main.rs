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
