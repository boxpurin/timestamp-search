pub mod timestamp;
pub mod video;

pub trait Index {
    fn pid(&self) -> Option<&str> {
        None
    }
}