#[macro_export]
#[doc(hidden)]
macro_rules! impl_numeric_value_traits {
    ($name:ident, $type:ty) => {
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl TryFrom<$type> for $name {
            type Error = errors::AppError;
            fn try_from(value: $type) -> errors::AppResult<Self> {
                Self::new(value)
            }
        }

        impl AsRef<$type> for $name {
            fn as_ref(&self) -> &$type {
                &self.0
            }
        }

        impl std::ops::Deref for $name {
            type Target = $type;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::str::FromStr for $name {
            type Err = errors::AppError;
            fn from_str(s: &str) -> errors::AppResult<Self> {
                let value = s.parse::<$type>().map_err(|_| {
                    errors::AppError::InvalidInput("Invalid seconds value".to_string())
                })?;
                Self::new(value)
            }
        }

        impl std::convert::From<$name> for String {
            fn from(value: $name) -> Self {
                value.to_string()
            }
        }

        impl std::convert::From<&$name> for String {
            fn from(value: &$name) -> Self {
                value.to_string()
            }
        }

        impl std::convert::From<$name> for $type {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl std::convert::From<&$name> for $type {
            fn from(value: &$name) -> Self {
                value.0
            }
        }

        impl std::cmp::PartialEq<$type> for $name {
            fn eq(&self, other: &$type) -> bool {
                self.0 == *other
            }
        }

        impl std::cmp::PartialEq<$name> for $type {
            fn eq(&self, other: &$name) -> bool {
                *self == other.0
            }
        }

        impl garde::rules::range::Bounds for $name {
            type Size = $type;

            const MIN: Self::Size = Self::Size::MIN;
            const MAX: Self::Size = Self::Size::MAX;
            // Required method
            fn validate_bounds(
                &self,
                lower_bound: Self::Size,
                upper_bound: Self::Size,
            ) -> Result<(), garde::rules::range::OutOfBounds> {
                match self.0 {
                    v if v < lower_bound => {
                        return Err(garde::rules::range::OutOfBounds::Lower);
                    }
                    v if v > upper_bound => {
                        return Err(garde::rules::range::OutOfBounds::Upper);
                    }
                    _ => {}
                }
                Ok(())
            }
        }
    };
}

#[macro_export]
macro_rules! impl_numeric_value {
    ($name:ident, $type:ty) => {
        #[derive(
            Debug,
            Clone,
            Copy,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            serde::Serialize,
            serde::Deserialize,
        )]
        pub struct $name(pub $type);

        impl $name {
            pub fn value(&self) -> $type {
                self.0
            }
        }

        $crate::impl_numeric_value_traits!($name, $type);
    };
}

#[cfg(test)]
mod unit_tests {
    use errors::AppResult;

    impl_numeric_value!(NumericValue, i32);
    impl NumericValue {
        pub fn new(value: i32) -> AppResult<Self> {
            Ok(Self(value))
        }
    }

    #[test]
    fn test_numeric_value() {
        let value = NumericValue::new(42).unwrap();
        assert_eq!(value.value(), 42);
        assert_eq!(value.to_string(), "42");

        let as_string: String = value.into();
        assert_eq!(as_string, "42");

        let deref_value: &i32 = &value;
        assert_eq!(*deref_value, 42);

        let from_value: i32 = value.into();
        assert_eq!(from_value, 42);
    }
}
