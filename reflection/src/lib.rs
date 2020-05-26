use serde::{Serialize, Deserialize, de, ser,
    ser::{SerializeMap, SerializeStruct},
};
use serde_reflection::{ContainerFormat, Format};
mod impl_reflection;

pub trait Reflection {
    /// Gives the format of a primitive, or in the case of a struct/enum, it will return
    /// `Format::Typename`
    fn get_format() -> Result<Format, String>;
    /// Only for structs/enums
    fn get_container_format() -> Result<ContainerFormat, String> {
        Err("Type is not a container".to_string())
    }
    fn register(registry: &mut serde_reflection::Registry) -> Result<(), String> {
        match Self::get_format()? {
            Format::TypeName(name) => {
                registry.insert(name, Self::get_container_format()?);
                Ok(())
            }
            format => Err(format!("Can only register structs and enums; got {:?}", format))
        }
    }
}
