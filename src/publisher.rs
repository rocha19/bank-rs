use crate::{command::Command, observer::Observer};

#[derive(Default)]
pub struct Publisher {
    observers: Vec<Box<dyn Observer>>,
}

impl Publisher {
    pub fn new() -> Self {
        Publisher {
            observers: Vec::new(),
        }
    }

    pub fn register(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    pub fn publish(&self, command: &dyn Command) {
        for observer in &self.observers {
            if observer.operation() == command.get_operation() {
                observer.notify(command);
            }
        }
    }
}
