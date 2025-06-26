
#[macro_export]
macro_rules! impl_public_string_value {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, Hash)]
        #[serde(rename_all = "camelCase")]
        pub struct $name(pub String);

        impl $name {
            pub fn new(value: String) -> Self {
                $name(value)
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> String {
                value.0
            }
        }

        impl From<String> for $name {
            /// Creates a new instance of `$name` from a `String`.
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl std::ops::Deref for $name {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::cmp::PartialEq<$name> for String {
            fn eq(&self, other: &$name) -> bool {
                self == &other.0
            }
        }

        impl std::cmp::PartialEq<$name> for &str {
            fn eq(&self, other: &TestStringValue) -> bool {
                self == &other.0
            }
        }

        impl std::cmp::PartialEq<String> for $name {
            fn eq(&self, other: &String) -> bool {
                self.0 == *other
            }
        }

        impl std::cmp::PartialEq<&str> for $name {
            fn eq(&self, other: &&str) -> bool {
                self.0 == *other
            }
        }

        impl std::str::FromStr for $name {
            type Err = std::string::ParseError;

            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Ok($name(s.to_string()))
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_string_value {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct $name(String);

        impl $name {
            pub fn new(value: String) -> Self {
                $name(value)
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> String {
                value.0
            }
        }

        impl From<String> for $name {
            /// Creates a new instance of `$name` from a `String`.
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl std::ops::Deref for $name {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::cmp::PartialEq<$name> for String {
            fn eq(&self, other: &$name) -> bool {
                self == &other.0
            }
        }

        impl std::cmp::PartialEq<String> for $name {
            fn eq(&self, other: &String) -> bool {
                self.0 == *other
            }
        }

        impl std::cmp::PartialEq<&str> for $name {
            fn eq(&self, other: &&str) -> bool {
                self.0 == *other
            }
        }


        impl std::str::FromStr for $name {
            type Err = std::string::ParseError;

            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Ok($name(s.to_string()))
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    impl_public_string_value!(TestStringValue);
    impl_string_value!(TestPrivateStringValue);

    #[test]
    fn test_string_value() {
        // From<String> implementation
        let value: TestStringValue = "Hello, World!".to_string().into();

        // From<TestStringValue> implementation
        let _: String = value.clone().into();

        // as_str method
        assert_eq!(value.as_str(), "Hello, World!");

        // PartialEq with TestStringValue
        let value2 = TestStringValue::new("Hello, World!".to_string());
        assert_eq!(value, value2);

        // PartialEq with String
        let string_value: String = value.clone().into();
        assert_eq!(value, string_value);

        // PartialEq with &str
        let str_value: &str = "Hello, World!";
        assert_eq!(value, str_value);

        // Display implementation
        assert_eq!(format!("{}", value), "Hello, World!");

        // Deref implementation
        let deref_value: &str = &value;
        assert_eq!(deref_value, "Hello, World!");

        // FromStr implementation
        let parsed_value: TestStringValue = "Hello, World!".parse().unwrap();
        assert_eq!(parsed_value, value);

        // AsRef implementation
        let as_ref_value: &str = value.as_ref();
        assert_eq!(as_ref_value, "Hello, World!");
    }

    #[test]
    fn test_private_string_value() {

        // From<String> implementation
        let value: TestPrivateStringValue = "Hello, World!".to_string().into();

        // From<TestStringValue> implementation
        let _: String = value.clone().into();

        // as_str method
        assert_eq!(value.as_str(), "Hello, World!");

        // PartialEq with TestStringValue
        let value2 = TestPrivateStringValue::new("Hello, World!".to_string());
        assert_eq!(value, value2);

        // PartialEq with String
        let string_value: String = value.clone().into();
        assert_eq!(value, string_value);

        // PartialEq with &str
        let str_value: &str = "Hello, World!";
        assert_eq!(value, str_value);

        // Display implementation
        assert_eq!(format!("{}", value), "Hello, World!");

        // Deref implementation
        let deref_value: &str = &value;
        assert_eq!(deref_value, "Hello, World!");

        // FromStr implementation
        let parsed_value: TestPrivateStringValue = "Hello, World!".parse().unwrap();
        assert_eq!(parsed_value, value);

        // AsRef implementation
        let as_ref_value: &str = value.as_ref();
        assert_eq!(as_ref_value, "Hello, World!");
    }
}