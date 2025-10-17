use errors::{AppResult, AppError};
types::impl_numeric_value!(Offset, usize);

impl Offset {
    pub fn new(per_page: usize) -> AppResult<Self> {
        if per_page == 0 {
            return Err(AppError::InvalidInput("invalid per page".into()))
        }

        if per_page > 100 {
            return Err(AppError::InvalidInput("Too large per page".into()))
        }

        Ok(Offset(per_page))
    }
}