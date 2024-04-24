use alua::*;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::ffi::{CStr, CString};
use std::rc::Rc;
use std::sync::Arc;

#[derive(ClassAnnotation)]
// This is the only way to expose methods.
#[alua(fields = [
    "method fun(self: Example, message: string) - Send a message",
])]
#[allow(unused)]
struct Example {
    /// Example docs
    u8_integer: u8,
    /// Example docs
    u16_integer: u16,
    /// Example docs
    u32_integer: u32,
    /// Example docs
    i8_integer: i8,
    /// Example docs
    i16_integer: i16,
    /// Example docs
    i32_integer: i32,
    /// Example docs
    f32_number: f32,
    /// Example docs
    f64_number: f64,
    /// Example docs
    str_slice: &'static str,
    /// Example docs
    string: String,
    /// Example docs
    cstr_slice: &'static CStr,
    /// Example docs
    cstring: CString,
    /// Example docs
    u32_slice: &'static [u32],
    /// Example docs
    u32_rc: Rc<u32>,
    /// Example docs
    u32_arc: Arc<u32>,
    /// Example docs
    cow_string: Cow<'static, str>,
    /// Example docs
    optional_string: Option<String>,
    /// Example docs
    hashmap: HashMap<String, u32>,
    /// Example docs
    hashset: HashSet<String>,
}

fn main() {
    print!("{}", Example::class_annotation());
}
