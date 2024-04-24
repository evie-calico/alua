use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::ffi::{CStr, CString};
use std::{rc::Rc, sync::Arc};

pub use alua_macros::*;

pub trait TypeAnnotation {
    fn lua_type() -> String;
}

pub trait ClassAnnotation {
    fn class_annotation() -> String;
}

/// Assign a static string to a given type.
macro_rules! simple {
    ($ident:ty, $name:expr) => {
        impl TypeAnnotation for $ident {
            fn lua_type() -> String {
                String::from($name)
            }
        }
    };
}

/// Add a suffix to the wrapped type.
macro_rules! suffix {
    ($ident:ty, $name:expr) => {
        impl<T: TypeAnnotation> TypeAnnotation for $ident {
            fn lua_type() -> String {
                T::lua_type() + $name
            }
        }
    };
}

/// Ignore this type, returning the wrapped type's value instead.
macro_rules! pass {
    ($ident:ty) => {
        impl<T: TypeAnnotation> TypeAnnotation for $ident{
            fn lua_type() -> String {
                T::lua_type()
            }
        }
    };
    ($($ident:ty),*) => {
        $(pass!($ident);)*
    }
}

/// Shortcut for types which convert to lua's integer.
macro_rules! integer {
    ($ident:ty) => {
        simple!($ident, "integer");
    };
    ($($ident:ty),*) => {
        $(integer!($ident);)*
    }
}

/// Shortcut for types which convert to lua's string.
macro_rules! string {
    ($ident:ty) => {
        simple!($ident, "string");
    };
    ($($ident:ty),*) => {
        $(string!($ident);)*
    }
}

// All types that mlua implements `IntoLua` for should also get `TypeAnnotation` implementations.

// `u64`, `u128`, and `i128` are left out because I don't know how luals represents them (if at all).
integer!(u8, u16, u32, i8, i16, i32, i64);

// mlua implements `IntoLua` for `BString`, too.
// Providing an implementation for `str` allows for `Box<str>`.
string!(String, &str, str, CString, &CStr);

simple!(f32, "number");
simple!(f64, "number");
simple!(bool, "boolean");

pass!(Box<T>, Rc<T>, Arc<T>);

suffix!(Option<T>, "?");
suffix!(Vec<T>, "[]");
suffix!([T], "[]");
suffix!(&[T], "[]");

impl<K: TypeAnnotation, V: TypeAnnotation> TypeAnnotation for HashMap<K, V> {
    fn lua_type() -> String {
        format!("table<{}, {}>", K::lua_type(), V::lua_type())
    }
}

impl<K: TypeAnnotation, V: TypeAnnotation> TypeAnnotation for BTreeMap<K, V> {
    fn lua_type() -> String {
        format!("table<{}, {}>", K::lua_type(), V::lua_type())
    }
}

impl<T: TypeAnnotation> TypeAnnotation for HashSet<T> {
    fn lua_type() -> String {
        format!("table<{}, bool>", T::lua_type())
    }
}

impl<T: TypeAnnotation> TypeAnnotation for BTreeSet<T> {
    fn lua_type() -> String {
        format!("table<{}, bool>", T::lua_type())
    }
}

impl<T: TypeAnnotation + Clone> TypeAnnotation for Cow<'_, T> {
    fn lua_type() -> String {
        T::lua_type()
    }
}

impl TypeAnnotation for Cow<'_, str> {
    fn lua_type() -> String {
        str::lua_type()
    }
}

impl<T: TypeAnnotation, const N: usize> TypeAnnotation for [T; N] {
    fn lua_type() -> String {
        T::lua_type() + "[]"
    }
}
