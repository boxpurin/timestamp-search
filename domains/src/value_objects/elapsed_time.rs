use errors::AppResult;
use types::impl_numeric_value;
impl_numeric_value!(ElapsedTime, u64);

impl ElapsedTime {
    pub fn new(seconds: u64) -> AppResult<Self> {
        Ok(ElapsedTime(seconds))
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn valid_seconds() {
        assert!(ElapsedTime::new(0).is_ok());
        assert!(ElapsedTime::new(1000).is_ok());
    }
}
