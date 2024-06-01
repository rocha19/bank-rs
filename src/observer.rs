use super::command::Command;

pub trait Observer {
    fn operation(&self) -> &str;
    fn notify(&self, command: &dyn Command);
}
