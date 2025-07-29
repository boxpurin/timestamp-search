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