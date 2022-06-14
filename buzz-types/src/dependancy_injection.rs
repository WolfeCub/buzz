use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub struct DependancyInjection {
    injectable: HashMap<TypeId, Box<dyn Any>>,
}

impl DependancyInjection {
    pub fn new() -> Self {
        Self {
            injectable: HashMap::new(),
        }
    }

    pub fn register<T: 'static>(&mut self, injectable: T) {
        self.injectable
            .insert(injectable.type_id(), Box::new(injectable));
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.injectable
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref())
    }
}
