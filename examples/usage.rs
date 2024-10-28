use alua::ClassAnnotation;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::ffi::{CStr, CString};
use std::rc::Rc;
use std::sync::Arc;

#[cfg(feature = "userdata")]
fn send(_lua: &mlua::Lua, _this: &mut Example, message: String) -> mlua::Result<()> {
    println!("-- {message}");
    Ok(())
}

// Unit enums can derive a "class annotation" that creates an alias for various string values.
// This may be useful for serde-based interop, rather than userdata.
// Enums have no derivable userdata, nor class annotations for a userdata implementation.
#[derive(Default, Clone, ClassAnnotation)]
#[allow(unused)]
enum ExampleVariant {
    #[default]
    One,
    Two,
    #[alua(skip)]
    Three,
    Four,
}

#[derive(Default, ClassAnnotation)]
#[cfg_attr(feature = "userdata", derive(alua_macros::UserData))]
#[cfg_attr(feature = "userdata", alua(method = send))]
// This is the only way to expose methods.
#[alua(
    fields = [
        "send fun(self: Example, message: string) - Send a message",
    ],
)]
#[allow(unused)]
struct Example {
    /// Example docs
    variant: ExampleVariant,
    /// Example docs
    #[cfg_attr(feature = "userdata", alua(get))]
    #[alua(as_lua = "string?")]
    retyped_u8_integer: Option<u8>,
    /// Example docs
    #[cfg_attr(feature = "userdata", alua(get, set))]
    u16_integer: u16,
    /// Example docs
    #[cfg_attr(feature = "userdata", alua(get, set))]
    u32_integer: u32,
    /// Example docs
    #[cfg_attr(feature = "userdata", alua(get, set))]
    i8_integer: i8,
    /// Example docs
    #[cfg_attr(feature = "userdata", alua(get, set))]
    i16_integer: i16,
    /// Example docs
    #[cfg_attr(feature = "userdata", alua(get, set))]
    i32_integer: i32,
    /// Example docs
    #[cfg_attr(feature = "userdata", alua(get, set))]
    f32_number: f32,
    /// Example docs
    #[cfg_attr(feature = "userdata", alua(get, set))]
    f64_number: f64,
    /// Example docs
    #[cfg_attr(feature = "userdata", alua(get))]
    str_slice: &'static str,
    /// Example docs
    #[cfg_attr(feature = "userdata", alua(get, set))]
    string: String,
    /// Example docs
    #[cfg_attr(feature = "userdata", alua(get))]
    cstr_slice: &'static CStr,
    /// Example docs
    #[cfg_attr(feature = "userdata", alua(get, set))]
    cstring: CString,
    /// Example docs
    #[cfg_attr(feature = "userdata", alua(get))]
    u32_slice: &'static [u32],
    /// Example docs
    #[cfg_attr(feature = "userdata", alua(get))]
    self_rc: Option<Rc<Example>>,
    /// Example docs
    #[cfg_attr(feature = "userdata", alua(get))]
    self_arc: Option<Arc<Example>>,
    /// Example docs
    #[cfg_attr(feature = "userdata", alua(get))]
    cow_string: Cow<'static, str>,
    /// Example docs
    #[cfg_attr(feature = "userdata", alua(get, set))]
    optional_string: Option<String>,
    /// Example docs
    #[cfg_attr(feature = "userdata", alua(get, set))]
    hashmap: HashMap<String, u32>,
    /// Example docs
    #[cfg_attr(feature = "userdata", alua(get, set))]
    hashset: HashSet<String>,
}

fn main() {
    println!("{}", ExampleVariant::class_annotation());
    print!("{}", Example::class_annotation());

    #[cfg(feature = "userdata")]
    {
        let lua = mlua::Lua::new();
        lua.globals()
            .set(
                "Example",
                Example {
                    retyped_u8_integer: Some(2),
                    ..Default::default()
                },
            )
            .unwrap();
        lua.load(mlua::chunk! {
            print("-- Lua Output")
            print("-- u32_integer: "..Example.u32_integer)
            print("-- string: \"\""..Example.string)
            print("-- changing values")
            Example.u32_integer = 255
            Example.string = "Hello, World!"
            print("-- u32_integer: "..Example.u32_integer)
            print("-- string: \""..Example.string.."\"")
            print("-- type of retyped value: "..type(Example.retyped_u8_integer))
            Example:send("Method call");
        })
        .exec()
        .unwrap();
    }
}
