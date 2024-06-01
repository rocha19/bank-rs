use crate::domain::entities::{
    command::Command, credit_command::CreditCommand, observer::Observer,
};

pub struct Publisher {
    observers: Vec<Box<dyn Observer>>,
}

impl Publisher {
    pub fn new() -> Publisher {
        Publisher {
            observers: Vec::new(),
        }
    }

    pub fn subscribe(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    pub fn publish(&self, command: Box<dyn Command>) {
        for observer in &self.observers {
            if let Some(credit_command) = command.as_any().downcast_ref::<CreditCommand>() {
                if observer.operation() == credit_command.operation() {
                    observer.notify(command); // Cloning the command to maintain ownership
                }
            }
        }
    }
}

pub trait Any {
    fn as_any(&self) -> &dyn std::any::Any;
}

impl<T: 'static> Any for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
