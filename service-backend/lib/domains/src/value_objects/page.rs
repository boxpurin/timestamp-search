use errors::{AppError, AppResult};
types::impl_numeric_value!(Page, usize);

impl Page {
    pub fn new(page: usize) -> AppResult<Self> {
        if page == 0 {
            return Err(AppError::InvalidInput("Page cannot be 0".into()));
        }

        Ok(Self(page))
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn valid_page() {
        assert!(Page::new(1).is_ok());
        assert!(Page::new(usize::MAX).is_ok());
    }

    #[test]
    fn invalid_page() {
        assert!(Page::new(0).is_err());
    }
}
