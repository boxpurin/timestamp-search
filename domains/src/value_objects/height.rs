use errors::{AppError, AppResult};
types::impl_numeric_value!(Height, u32);
impl Height {
    pub fn new(height: u32) -> AppResult<Self> {
        if height == 0 {
            return Err(AppError::InvalidInput("Height cannot be zero".to_string()));
        }
        Ok(Height(height))
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use errors::AppError;

    #[test]
    fn test_height() {
        assert!(Height::new(0).is_err());
        assert!(matches!(Height::new(0), Err(AppError::InvalidInput(_))));
        assert!(Height::new(100).is_ok());

        let height = Height::new(200).unwrap();
        assert_eq!(height.0, 200);
    }
}
