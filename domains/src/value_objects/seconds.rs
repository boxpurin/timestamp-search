use types::impl_numeric_value;
impl_numeric_value!(Seconds, u64);

impl Seconds {
    pub fn new(seconds: u64) -> Self {
        Seconds(seconds)
    }
}
