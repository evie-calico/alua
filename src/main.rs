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
