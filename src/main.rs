use alua::*;

#[derive(ClassAnnotation)]
#[alua(functions = [
    "find_code_references(code string) CodeReferenceWithDocument[]",
    "find_discrete_values(unique_id string) DiscreteValue[]",
    "find_medications(external_id string) Medication[]",
    "find_documents(document_id string) CACDocument[]",
])]
struct Example {
    /// test
    /// meow
    #[allow(unused)]
    field: u32,
}

fn main() {
    print!("{}", Example::class_annotation());
}
