use errors::{AppError, AppResult};
use types::impl_numeric_value;
impl_numeric_value!(Seconds, u64);

impl Seconds {
    pub fn new(seconds: u64) -> AppResult<Self> {
        Ok(Seconds(seconds))
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_seconds() {
        assert!(Seconds::new(0).is_ok());
        assert!(Seconds::new(100).is_ok());

        let seconds = Seconds::new(200).unwrap();
        assert_eq!(seconds.0, 200);
    }
}