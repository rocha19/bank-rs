use std::any::Any;

pub trait Command: Any {
    fn operation(&self) -> &str;
}
