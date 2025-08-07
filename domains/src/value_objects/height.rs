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

    #[rstest::rstest]
    #[test]
    #[case(vec![180, 240, 360, 480, 720])]
    fn valid_height(#[case] heights: Vec<u32>) {
        for height in heights {
            assert!(Height::new(height).is_ok());

            let h = Height::new(height).unwrap();
            assert_eq!(h, height);
        }
    }

    #[rstest::rstest]
    #[test]
    #[case(0)]
    fn invalid_height(#[case] height: usize) {
        assert!(Height::new(0).is_err());
    }
}
