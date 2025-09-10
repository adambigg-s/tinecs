use std::any::Any;
use std::any::TypeId;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::LazyLock;
use std::sync::Mutex;
use std::sync::MutexGuard;

static MASTER: LazyLock<Mutex<Master>> = LazyLock::new(|| Mutex::new(Master::default()));

#[track_caller]
#[inline(always)]
pub fn master() -> MutexGuard<'static, Master> {
    /* TODO: */
    /* def need to handle this error, just for prototyping */
    MASTER.lock().expect("failed to lock master")
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Entity {
    ident: u32,
}

impl Entity {
    pub fn build<Id>(id: Id) -> Self
    where
        Id: Into<u32>,
    {
        Self { ident: Into::into(id) }
    }

    pub fn id(&self) -> u32 {
        **self
    }
}

impl Deref for Entity {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.ident
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
    Self: Any + Send + Sync,
{
}

trait AsAny {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl AsAny for dyn Component {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub trait System {
    fn run(&mut self);
}

#[derive(Default)]
pub struct Master {
    curr_ident: u32,
    components: HashMap<TypeId, HashMap<Entity, Box<dyn Component>>>,
}

impl Master {
    pub fn create_entity(&mut self) -> Entity {
        let out = Entity::build(self.curr_ident);
        self.curr_ident += 1;
        out
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        self.components.values_mut().for_each(|component| {
            component.remove(&entity);
        });
    }

    pub fn add_component<C>(&mut self, entity: Entity, component: C)
    where
        C: Component,
    {
        let type_ident = TypeId::of::<C>();
        let boxed = Box::new(component);

        self.components.entry(type_ident).or_default().insert(entity, boxed);
    }

    #[cfg(never)]
    pub fn remove_component<C>(&mut self, entity: Entity) -> Option<C>
    where
        C: Component,
    {
        let type_ident = TypeId::of::<C>();
        let boxed = self.components.get_mut(&type_ident)?.remove(&entity)?;
        /* WARNING: */
        /* wait, this might not even be possible without just cloning it... */

        if let Some(out) = boxed.as_any().downcast_ref().map(|value| *value) {
            return Some(out);
        }
        None
    }

    pub fn remove_component_dont_return<C>(&mut self, entity: Entity)
    where
        C: Component,
    {
        let type_ident = TypeId::of::<C>();

        self.components.get_mut(&type_ident).and_then(|outer| outer.remove(&entity));
    }

    pub fn get_component<C>(&self, entity: Entity) -> Option<&C>
    where
        C: Component,
    {
        let type_ident = TypeId::of::<C>();

        /* this line is kind of ridiculous */
        // basically, grab the original type (outer hashmap) based on the
        // generic C, then take the actual data stored on Id. this has to be
        // recast to the C's type, so cast as Any and re(down)cast to C
        let Some(type_layer) = self.components.get(&type_ident)
        else {
            panic!("no type added yet");
        };
        let Some(entity_layer) = type_layer.get(&entity)
        else {
            panic!("no components added yet");
        };
        /* TODO: */
        /* none of these should actually panic, just for prototype */
        // better implementation below
        let Some(out) = entity_layer.as_any().downcast_ref::<C>()
        else {
            panic!("failed to downcast component");
        };

        Some(out)
    }

    pub fn get_component_mut<C>(&mut self, entity: Entity) -> Option<&mut C>
    where
        C: Component,
    {
        let type_ident = TypeId::of::<C>();

        self.components
            .get_mut(&type_ident)
            .and_then(|outer| outer.get_mut(&entity))
            .and_then(|inner| inner.as_any_mut().downcast_mut::<C>())
    }
}

#[cfg(test)]
mod glmutable_test {
    use std::hint::black_box;

    use super::*;

    #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct FooBarStruct {
        field: i32,
    }

    impl Component for FooBarStruct {}

    #[test]
    #[allow(clippy::unit_cmp)]
    fn glmut_init() {
        let master = master();
        black_box(&master);
        assert!(drop(master) == ());
    }

    #[test]
    fn glmut_insert() {
        let mut master = master();
        let ent = master.create_entity();
        let cmp = FooBarStruct::default();
        master.add_component(ent, cmp);
        assert!(master.components.len() == 1);
    }

    #[test]
    fn glmut_get() {
        let mut master = master();
        let ent = master.create_entity();
        let cmp = FooBarStruct::default();
        master.add_component(ent, cmp);
        assert!(*master.get_component::<FooBarStruct>(ent).unwrap() == FooBarStruct::default());
    }

    #[test]
    fn glmut_get_mut() {
        let mut master = master();
        let ent = master.create_entity();
        let cmp = FooBarStruct::default();
        master.add_component(ent, cmp);

        let query = master.get_component_mut::<FooBarStruct>(ent);
        assert!(query.is_some());

        query.unwrap().field = 33;
        assert!(master.get_component::<FooBarStruct>(ent).unwrap().field == 33);
    }
}
