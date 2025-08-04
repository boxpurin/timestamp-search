pub mod timestamp;
pub mod video;

pub trait Index {
    fn pid(&self) -> Option<&str> {
        None
    }

    fn pid_field() -> Option<&'static str> {
        None
    }

    fn name() -> &'static str;
}
