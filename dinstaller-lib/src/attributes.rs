//! This module offers a mechanism to easily map the values from the command
//! line to proper installation settings.
//!
//! To specify a value in the command line, the user needs specify:
//!
//! * a setting ID (`"users.name"`, `"storage.lvm"`, and so on), that must be used to find the
//!   setting within a [super::settings::Settings] struct.
//! * a value, which is captured as a string (`"Foo Bar"`, `"true"`, etc.) and it should be
//!   converted to the proper type.
//!
//! Implementing the [Attributes] trait adds support for setting the value in an straightforward,
//! taking care of the conversions automatically. The newtype [AttributeValue] takes care of such a
//! conversion.
//!
/// For plain structs, the implementation can be derived.
///
/// TODO: derive for top-level structs too
use std::convert::TryFrom;

/// Implements support for easily settings attributes values given an ID (`"users.name"`) and a
/// string value (`"Foo bar"`).
///
/// In the example below, the trait is manually implemented for `Settings and derived for
/// `UserSettings`.
///
/// ```
/// # use dinstaller_derive::DInstallerAttributes;
/// # use dinstaller_lib::attributes::{Attributes, AttributeValue};
///
/// #[derive(DInstallerAttributes)]
/// struct UserSettings {
///   name: String,
///   enabled: bool
/// }
///
/// struct Settings {
///   user: UserSettings
/// }
///
/// impl Attributes for Settings {
///   fn set_attribute(&mut self, attr: &str, value: AttributeValue) -> Result<(), &'static str> {
///     if let Some((ns, id)) = attr.split_once('.') {
///       match ns {
///         "user" => self.user.set_attribute(id, value)?,
///         _ => return Err("unknown attribute")
///       }
///     }
///     Ok(())
///   }
/// }
///
/// let user = UserSettings { name: "foo".to_string(), enabled: false };
/// let mut settings = Settings { user };
/// settings.set_attribute("user.name", AttributeValue("foo.bar".to_string()));
/// settings.set_attribute("user.enabled", AttributeValue("true".to_string()));
/// assert!(&settings.user.enabled);
/// assert_eq!(&settings.user.name, "foo.bar");
/// ```
pub trait Attributes {
    fn set_attribute(&mut self, attr: &str, value: AttributeValue) -> Result<(), &'static str>;
}

/// Represents a string-based value and allows converting them to other types
///
/// Supporting more conversions if a matter of implementing the [std::convert::TryFrom] trait for
/// more types.
///
/// ```
///   # use dinstaller_lib::attributes::AttributeValue;
//
///   let value = AttributeValue("true".to_string());
///   let value: bool = value.try_into().expect("the conversion failed");
///   assert_eq!(value, true);
/// ```
pub struct AttributeValue(pub String);

impl TryFrom<AttributeValue> for bool {
    type Error = &'static str;

    fn try_from(value: AttributeValue) -> Result<Self, Self::Error> {
        match value.0.to_lowercase().as_str() {
            "true" | "yes" | "t" => Ok(true),
            "false" | "no" | "f" => Ok(false),
            _ => Err("not a valid boolean"),
        }
    }
}

impl TryFrom<AttributeValue> for String {
    type Error = &'static str;

    fn try_from(value: AttributeValue) -> Result<Self, Self::Error> {
        Ok(value.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_bool() {
        let value = AttributeValue("true".to_string());
        let value: bool = value.try_into().unwrap();
        assert_eq!(value, true);

        let value = AttributeValue("false".to_string());
        let value: bool = value.try_into().unwrap();
        assert_eq!(value, false);
    }

    #[test]
    fn test_try_from_string() {
        let value = AttributeValue("some value".to_string());
        let value: String = value.try_into().unwrap();
        assert_eq!(value, "some value");
    }
}
