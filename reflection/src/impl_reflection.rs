//! Default implementations of Reflection
use crate::Reflection;
use std::collections::{HashMap, BTreeMap};
use either::*;
use serde_reflection::{ContainerFormat, Format};

// Primitive types
impl Reflection for () {
    fn get_format() -> Result<Format, String> {
        Ok(Format::Unit)
    }
}
impl Reflection for bool {
    fn get_format() -> Result<Format, String> {
        Ok(Format::Bool)
    }
}

impl Reflection for i8 {
    fn get_format() -> Result<Format, String> {
        Ok(Format::I8)
    }
}
impl Reflection for i16 {
    fn get_format() -> Result<Format, String> {
        Ok(Format::I16)
    }
}
impl Reflection for i32 {
    fn get_format() -> Result<Format, String> {
        Ok(Format::I32)
    }
}
impl Reflection for i64 {
    fn get_format() -> Result<Format, String> {
        Ok(Format::I64)
    }
}
impl Reflection for i128 {
    fn get_format() -> Result<Format, String> {
        Ok(Format::I128)
    }
}
impl Reflection for u8 {
    fn get_format() -> Result<Format, String> {
        Ok(Format::U8)
    }
}
impl Reflection for u16 {
    fn get_format() -> Result<Format, String> {
        Ok(Format::U16)
    }
}
impl Reflection for u32 {
    fn get_format() -> Result<Format, String> {
        Ok(Format::U32)
    }
}
impl Reflection for u64 {
    fn get_format() -> Result<Format, String> {
        Ok(Format::U64)
    }
}
impl Reflection for u128 {
    fn get_format() -> Result<Format, String> {
        Ok(Format::U128)
    }
}
impl Reflection for f32 {
    fn get_format() -> Result<Format, String> {
        Ok(Format::F32)
    }
}
impl Reflection for f64 {
    fn get_format() -> Result<Format, String> {
        Ok(Format::F64)
    }
}
impl Reflection for char {
    fn get_format() -> Result<Format, String> {
        Ok(Format::Char)
    }
}
impl Reflection for String {
    fn get_format() -> Result<Format, String> {
        Ok(Format::Str)
    }
}
impl Reflection for &str {
    fn get_format() -> Result<Format, String> {
        Ok(Format::Str)
    }
}

// Generic types
impl<T: Reflection> Reflection for &T {
    fn get_format() -> Result<Format, String> {
        T::get_format()
    }
}
impl<T: Reflection> Reflection for Option<T> {
    fn get_format() -> Result<Format, String> {
        Ok(Format::Option(Box::new(T::get_format()?)))
    }
}
impl<T: Reflection> Reflection for Vec<T> {
    fn get_format() -> Result<Format, String> {
        Ok(Format::Seq(Box::new(T::get_format()?)))
    }
}
impl<K: Reflection, V: Reflection> Reflection for HashMap<K, V> {
    fn get_format() -> Result<Format, String> {
        Ok(Format::Map {
            key: Box::new(K::get_format()?),
            value: Box::new(V::get_format()?),
        })
    }
}

impl<K: Reflection, V: Reflection> Reflection for BTreeMap<K, V> {
    fn get_format() -> Result<Format, String> {
        Ok(Format::Map {
            key: Box::new(K::get_format()?),
            value: Box::new(V::get_format()?),
        })
    }
}
