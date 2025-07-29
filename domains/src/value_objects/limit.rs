types::impl_numeric_value!(Limit, usize);
impl Limit {
    pub fn new(limit: usize) -> Self {
        Limit(limit)
    }
    
    pub fn parse(value: &str) -> Result<Self, String> {
        value.parse::<usize>()
            .map(Limit)
            .map_err(|_| format!("Invalid limit value: {}", value))
    }
}