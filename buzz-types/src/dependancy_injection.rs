use std::{
    any::{Any, TypeId},
    collections::HashMap, sync::{Arc, RwLock},
};

pub struct DependancyInjection {
    injectable: HashMap<TypeId, Arc<RwLock<dyn Any>>>,
}

impl DependancyInjection {
    pub fn new() -> Self {
        Self {
            injectable: HashMap::new(),
        }
    }

    pub fn register<T: 'static>(&mut self, injectable: T) {
        self.injectable
            .insert(injectable.type_id(), Arc::new(RwLock::new(Box::new(injectable))));
    }

    pub fn get<T: 'static>(&self) -> Option<&Arc<RwLock<dyn Any>>> {
        self.injectable
            .get(&TypeId::of::<T>())
    }
}
