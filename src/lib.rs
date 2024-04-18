use std::{rc::Rc, sync::Arc};

pub use alua_macros::*;

pub trait TypeAnnotation {
    fn lua_type() -> String;
}

pub trait ClassAnnotation {
    fn class_annotation() -> String;
}

macro_rules! simple {
    ($ident:ty, $name:expr) => {
        impl TypeAnnotation for $ident {
            fn lua_type() -> String {
                String::from($name)
            }
        }
    };
}

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

macro_rules! integer {
    ($ident:ty) => {
        simple!($ident, "integer");
    };
    ($($ident:ty),*) => {
        $(integer!($ident);)*
    }
}

integer!(u8, u16, u32, i8, i16, i32, i64);

simple!(f32, "number");
simple!(f64, "number");
simple!(bool, "bool");
simple!(String, "string");

pass!(Rc<T>);
pass!(Arc<T>);

impl<T: TypeAnnotation> TypeAnnotation for Option<T> {
    fn lua_type() -> String {
        T::lua_type() + "?"
    }
}

impl<T: TypeAnnotation> TypeAnnotation for Vec<T> {
    fn lua_type() -> String {
        T::lua_type() + "[]"
    }
}
