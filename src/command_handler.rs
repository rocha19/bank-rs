use super::command::Command;

pub trait CommandHandler {
    fn operation(&self) -> &str;
    fn notify(&self, command: &dyn Command);
}
