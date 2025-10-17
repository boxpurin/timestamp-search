use errors::AppResult;
types::impl_numeric_value!(PerPage, usize);

impl PerPage {
    pub fn new(per_page: usize) -> AppResult<Self> {
        Ok(Self(per_page))
    }
}