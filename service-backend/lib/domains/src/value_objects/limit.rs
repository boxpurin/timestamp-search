use errors::{AppError, AppResult};
types::impl_numeric_value!(Limit, usize);

static LIMIT_MIN: usize = 1;
static LIMIT_MAX: usize = 1000;

impl Limit {
    pub fn new(limit: usize) -> AppResult<Self> {
        if limit < LIMIT_MIN {
            return Err(AppError::InvalidInput(format!(
                "Limit must be greater than {}",
                LIMIT_MIN
            )));
        }

        if limit > LIMIT_MAX {
            return Err(AppError::InvalidInput(format!(
                "Limit must not exceed {}",
                LIMIT_MAX
            )));
        }

        Ok(Limit(limit))
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[test]
    #[case(vec![1, 5, 10, 25, 50, 100, 1000])]
    fn valid_limit(#[case] limits: Vec<usize>) {
        for limit in limits {
            assert!(Limit::new(limit).is_ok());
        }
    }

    #[rstest]
    #[test]
    #[case(vec![0, 1001])]
    fn invalid_limit(#[case] limits: Vec<usize>) {
        for limit in limits {
            assert!(Limit::new(limit).is_err());
        }
    }
}
