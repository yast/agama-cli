use std::convert::TryFrom;

pub struct AttributeValue(pub String);

pub trait Attributes {
    fn set_attribute(&mut self, attr: &str, value: AttributeValue) -> Result<(), &'static str>;
}

impl TryFrom<AttributeValue> for bool {
    type Error = &'static str;

    fn try_from(value: AttributeValue) -> Result<Self, Self::Error> {
        match value.0.to_lowercase().as_str() {
            "true" | "yes" | "t" => Ok(true),
            "false" | "no" | "f" => Ok(false),
            _ => Err("not a valid boolean")
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
