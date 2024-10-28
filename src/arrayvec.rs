use std::borrow::Cow;

use super::TypeAnnotation;
use arrayvec::{ArrayString, ArrayVec};

impl<T: TypeAnnotation, const CAP: usize> TypeAnnotation for ArrayVec<T, CAP> {
    fn lua_type() -> Cow<'static, str> {
        T::lua_type() + "[]"
    }
}

impl<const CAP: usize> TypeAnnotation for ArrayString<CAP> {
    fn lua_type() -> Cow<'static, str> {
        Cow::Borrowed("string")
    }
}
