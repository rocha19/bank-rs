pub trait Command {
    fn get_operation(&self) -> &str;
    fn as_any(&self) -> &dyn std::any::Any;
}
