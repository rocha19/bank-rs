use super::command::Command;

pub trait Observer {
    fn notify(&self, command: Box<dyn Command>);
    fn operation(&self) -> &str;
}
