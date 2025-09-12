use std::any::Any;
use std::any::TypeId;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::LazyLock;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::vec;

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

    pub fn id(self) -> u32 {
        *self
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

pub trait Component: Any + Send + Sync {}

trait AsAny {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl AsAny for dyn Component {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
}

pub struct Query<'d, C> {
    inner: Vec<&'d C>,
}

impl<'d, C> IntoIterator for Query<'d, C> {
    type Item = &'d C;

    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'d, C> SystemArg<'d> for Query<'d, C>
where
    C: Component,
{
    fn fetch(master: &'d Master) -> Self {
        master.query_components::<C>()
    }
}

pub struct QueryMut<'d, C> {
    inner: Vec<&'d mut C>,
}

impl<'d, C> IntoIterator for QueryMut<'d, C> {
    type Item = &'d mut C;

    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

/* can only fetch stuff immut right now, need to figure that out later */
// impl<'d, C> SystemArg<'d> for QueryMut<'d, C>
// where
//     C: Component,
// {
//     fn fetch(master: &'d Master) -> Self {
//         master.query_components_mut::<C>()
//     }
// }

pub trait SystemArg<'d>
where
    Self: Send + Sync + Sized,
{
    fn fetch(master: &'d Master) -> Self;
}

pub trait System
where
    Self: Send + Sync,
{
    fn run(&mut self, master: &mut Master);
}

pub struct ArgSet<T> {
    args: T,
}

impl<'d> SystemArg<'d> for ArgSet<()> {
    fn fetch(_: &'d Master) -> Self {
        Self { args: () }
    }
}

impl<'d, A> SystemArg<'d> for ArgSet<A>
where
    A: SystemArg<'d>,
{
    fn fetch(master: &'d Master) -> Self {
        Self { args: A::fetch(master) }
    }
}

impl<'d, A, B> SystemArg<'d> for ArgSet<(A, B)>
where
    A: SystemArg<'d>,
    B: SystemArg<'d>,
{
    fn fetch(master: &'d Master) -> Self {
        let a = A::fetch(master);
        let b = B::fetch(master);
        Self { args: (a, b) }
    }
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
        self.components
            .get(&type_ident)
            .and_then(|outer| outer.get(&entity))
            .and_then(|inner| inner.as_any().downcast_ref())
    }

    pub fn get_component_mut<C>(&mut self, entity: Entity) -> Option<&mut C>
    where
        C: Component,
    {
        let type_ident = TypeId::of::<C>();

        // exactly the same as above but does everything mutably
        self.components
            .get_mut(&type_ident)
            .and_then(|outer| outer.get_mut(&entity))
            .and_then(|inner| inner.as_any_mut().downcast_mut::<C>())
    }

    pub fn query_entities<C>(&self) -> impl Iterator<Item = Entity>
    where
        C: Component,
    {
        let type_ident = TypeId::of::<C>();

        self.components.get(&type_ident).into_iter().flat_map(|outer| outer.keys().copied())
    }

    pub fn query_components<'d, C>(&'d self) -> Query<'d, C>
    where
        C: Component,
    {
        let type_ident = TypeId::of::<C>();

        Query {
            inner: self
                .components
                .get(&type_ident)
                .into_iter()
                .flat_map(|outer| outer.values())
                .filter_map(|inner| inner.as_any().downcast_ref())
                .collect(),
        }
    }

    pub fn query_components_mut<'d, C>(&'d mut self) -> QueryMut<'d, C>
    where
        C: Component,
    {
        let type_ident = TypeId::of::<C>();

        QueryMut {
            inner: self
                .components
                .get_mut(&type_ident)
                .into_iter()
                .flat_map(|outer| outer.values_mut())
                .filter_map(|inner| inner.as_any_mut().downcast_mut())
                .collect(),
        }
    }

    #[allow(dead_code)]
    unsafe fn reset(&mut self) {
        *self = Self::default()
    }
}

#[cfg(test)]
mod glmutable_test {
    use std::hint::black_box;

    use super::*;

    impl Component for FooBarStruct {}
    #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct FooBarStruct {
        field: i32,
    }

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
        unsafe {
            master.reset();
        }
        let ent = master.create_entity();
        let cmp = FooBarStruct::default();
        master.add_component(ent, cmp);
        assert!(master.components.len() == 1);
    }

    #[test]
    fn glmut_destroy() {
        let mut master = master();
        unsafe {
            master.reset();
        }
        let ent = master.create_entity();
        let cmp = FooBarStruct::default();
        master.add_component(ent, cmp);
        master.destroy_entity(ent);
        assert!(master.get_component::<FooBarStruct>(ent).is_none());
    }

    #[test]
    fn glmut_get() {
        let mut master = master();
        unsafe {
            master.reset();
        }
        let ent = master.create_entity();
        let cmp = FooBarStruct::default();
        master.add_component(ent, cmp);
        assert!(*master.get_component::<FooBarStruct>(ent).unwrap() == FooBarStruct::default());
    }

    #[test]
    fn glmut_get_mut() {
        let mut master = master();
        unsafe {
            master.reset();
        }
        let ent = master.create_entity();
        let cmp = FooBarStruct::default();
        master.add_component(ent, cmp);

        let query = master.get_component_mut::<FooBarStruct>(ent);
        assert!(query.is_some());

        query.unwrap().field = 33;
        assert!(master.get_component::<FooBarStruct>(ent).unwrap().field == 33);
    }

    #[test]
    fn glmut_query() {
        let mut master = master();
        unsafe {
            master.reset();
        }
        let ent1 = master.create_entity();
        let cmp = FooBarStruct::default();
        master.add_component(ent1, cmp);

        {
            let query = master.query_entities::<FooBarStruct>();
            let counter = query.count();
            assert!(counter == 1, "counted: {}", counter);
        }

        let ent2 = master.create_entity();
        master.add_component(ent2, cmp);

        {
            let query = master.query_entities::<FooBarStruct>();
            let counter = query.count();
            assert!(counter == 2, "counted: {}", counter);
        }
    }
}
