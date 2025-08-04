use errors::{AppError, AppResult};
types::impl_numeric_value!(Width, u32);

impl Width {
    pub fn new(width: u32) -> AppResult<Self> {
        Ok(Width(width))
    }
}
