use std::{
    collections::HashMap,
    hint::black_box,
    marker::PhantomData,
    ops::Deref,
    sync::{LazyLock, Mutex, MutexGuard},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
struct Entity {
    id: u32,
}

impl Entity {
    pub fn build<Id>(id: Id) -> Self
    where
        Id: Into<u32>,
    {
        Self { id: Into::into(id) }
    }
}

impl Deref for Entity {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl<T> From<T> for Entity
where
    T: Into<u32>,
{
    fn from(value: T) -> Self {
        Self::build(value.into())
    }
}

pub trait Component
where
    Self: 'static + Send + Sync,
{
}

struct RecastMetadata<T> {
    former: PhantomData<T>,
}

impl<T> RecastMetadata<T> {
    fn recast<Comp>(&self, data: Comp) -> T
    where
        Comp: Component,
    {
        Into::into(data)
    }
}

pub trait System {
    fn run(&mut self);
}

#[derive(Default)]
pub struct Master {
    curr_id: u32,
    components: HashMap<Entity, Box<dyn Component>>,
}

impl Master {
    fn blackbox(&self) {
        black_box(self);
    }

    fn type_info(&self) {}

    fn create_entity(&mut self) -> Entity {
        let out = Entity::build(self.curr_id);
        self.curr_id += 1;
        out
    }

    fn add_component<Id, Comp>(&mut self, entity: Id, component: Comp)
    where
        Comp: Component,
        Id: Into<Entity>,
    {
        let boxed = Box::from(component);
        self.components.insert(entity.into(), boxed);
    }

    fn get_component<Id>(&mut self, entity: Id) -> Option<&Box<dyn Component>>
    where
        Id: Into<Entity>,
    {
        self.components.get(&entity.into())
    }
}

static MASTER: LazyLock<Mutex<Master>> = LazyLock::new(|| Mutex::new(Master::default()));

fn master() -> MutexGuard<'static, Master> {
    /* TODO: */
    /* def need to handle this error, just for prototyping */
    MASTER.lock().expect("failed to lock master")
}

#[cfg(test)]
mod glmutable_test {
    use super::*;

    #[test]
    #[allow(clippy::unit_cmp)]
    fn glmut_init() {
        master().blackbox();
        assert!(drop(master()) == ());
    }

    #[test]
    fn glmut_insert() {
        #[derive(Default)]
        struct FooBarStruct {
            _foo: f32,
            _bar: i32,
        }
        impl Component for FooBarStruct {}

        let entity = master().create_entity();
        let component = FooBarStruct::default();
        master().add_component(entity, component);
        assert!(master().components.len() == 1);
    }
}
