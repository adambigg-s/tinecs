pub mod arguments;
pub mod systems;

use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::HashMap,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    sync::{LazyLock, Mutex, MutexGuard},
};

use crate::{
    arguments::{Query, QueryMut},
    systems::SystemBuilder,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Entity {
    id: usize,
}

pub trait Component: Any + Send + Sync {}

pub trait RepresentAsAny {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl RepresentAsAny for dyn Component {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
}

pub trait System
where
    Self: Send + Sync,
{
    fn run(&mut self, map: &mut ComponentMap);
}

static MASTER: LazyLock<Mutex<Master>> = LazyLock::new(|| Mutex::new(Master::default()));

pub fn master() -> MutexGuard<'static, Master> {
    match MASTER.try_lock() {
        | Ok(master) => master,
        | Err(error) => {
            panic!("reference aliasing issue: {}", error);
        }
    }
}

pub fn master_check() -> bool {
    !MASTER.is_poisoned() && MASTER.lock().is_ok()
}

#[derive(Default)]
pub struct ComponentMap {
    inner: HashMap<TypeId, HashMap<Entity, RefCell<Box<dyn Component + 'static>>>>,
}

impl ComponentMap {
    fn query_components<'d, C>(&'d self) -> Query<'d, C>
    where
        C: 'static,
    {
        Query {
            inner: self
                .get(&TypeId::of::<C>())
                .into_iter()
                .flat_map(|outer| outer.values())
                .map(|inner| inner.borrow())
                .collect(),
            marker: PhantomData,
        }
    }

    fn query_components_mut<'d, C>(&'d self) -> QueryMut<'d, C>
    where
        C: 'static,
    {
        QueryMut {
            inner: self
                .get(&TypeId::of::<C>())
                .into_iter()
                .flat_map(|outer| outer.values())
                .map(|inner| inner.borrow_mut())
                .collect(),
            marker: PhantomData,
        }
    }
}

impl Deref for ComponentMap {
    type Target = HashMap<TypeId, HashMap<Entity, RefCell<Box<dyn Component>>>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for ComponentMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[derive(Default)]
pub struct Master {
    curr_entity: Entity,
    components: ComponentMap,
    systems: Vec<Box<dyn System>>,
}

impl Master {
    pub fn create_entity(&mut self) -> Entity {
        let out = self.curr_entity;
        self.curr_entity.id += 1;
        out
    }

    pub fn run(&mut self) {
        for system in self.systems.iter_mut() {
            system.run(&mut self.components);
        }
    }

    pub fn add_system<A, I, S>(&mut self, system: A)
    where
        A: SystemBuilder<I, System = S>,
        S: System + 'static,
    {
        self.systems.push(Box::new(system.build_system()));
    }

    pub fn add_component<C>(&mut self, entity: Entity, component: C)
    where
        C: Component + 'static,
    {
        self.components
            .inner
            .entry(TypeId::of::<C>())
            .or_default()
            .insert(entity, RefCell::new(Box::new(component)));
    }
}

#[cfg(test)]
mod tests {}
