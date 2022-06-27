use std::{
    any::{Any, TypeId},
    collections::HashMap, sync::RwLock,
};

pub struct DependancyInjection {
    injectable: HashMap<TypeId, RwLock<Box<dyn Any>>>,
}

impl DependancyInjection {
    pub fn new() -> Self {
        Self {
            injectable: HashMap::new(),
        }
    }

    pub fn register<T: 'static>(&mut self, injectable: T) {
        self.injectable
            .insert(injectable.type_id(), RwLock::new(Box::new(injectable)));
    }

    pub fn get<T: 'static>(&self) -> Option<&RwLock<Box<dyn Any>>> {
        self.injectable
            .get(&TypeId::of::<T>())
    }
}
