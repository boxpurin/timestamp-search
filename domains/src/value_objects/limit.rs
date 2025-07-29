use errors::{AppResult, AppError};
types::impl_numeric_value!(Limit, usize);
impl Limit {
    pub fn new(limit: usize) -> AppResult<Self> {
        if limit == 0 {
            return Err(AppError::InvalidInput("Limit must be greater than zero".to_string()));
        }

        if limit > 1000 {
            return Err(AppError::InvalidInput("Limit must not exceed 1000".to_string()));
        }

        Ok(Limit(limit))
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use errors::AppError;

    #[test]
    fn test_limit() {
        assert!(Limit::new(0).is_err());
        assert!(matches!(Limit::new(0), Err(AppError::InvalidInput(_))));
        assert!(Limit::new(1001).is_err());
        assert!(matches!(Limit::new(1001), Err(AppError::InvalidInput(_))));
        assert!(Limit::new(500).is_ok());

        let limit = Limit::new(200).unwrap();
        assert_eq!(limit.0, 200);
    }
}