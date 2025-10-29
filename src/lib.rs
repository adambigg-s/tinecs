use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Entity {
    id: u32,
}

pub trait Component: Any {}

pub trait System {
    fn run(&mut self, map: &mut ComponentMap);
}

pub trait SystemBuilder<In> {
    type System: System;

    fn build_system(self) -> Self::System;
}

pub struct FnSystem<In, Func> {
    func: Func,
    marker: PhantomData<fn() -> In>,
}

trait SystemArg {
    type Item<'o>;

    fn fetch<'i>(components: &'i ComponentMap) -> Self::Item<'i>;
}

#[derive(Default)]
pub struct ComponentMap {
    inner: HashMap<TypeId, HashMap<Entity, RefCell<Box<dyn Any + 'static>>>>,
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
        todo!()
    }
}

impl Deref for ComponentMap {
    type Target = HashMap<TypeId, HashMap<Entity, RefCell<Box<dyn Any>>>>;

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
        C: 'static,
    {
        self.components
            .inner
            .entry(TypeId::of::<C>())
            .or_default()
            .insert(entity, RefCell::new(Box::new(component)));
    }
}

impl<Func> System for FnSystem<((),), Func>
where
    for<'a> &'a mut Func: FnMut(),
{
    fn run(&mut self, _: &mut ComponentMap) {
        call_func(&mut self.func);

        fn call_func<F>(mut func: F)
        where
            F: FnMut(),
        {
            func()
        }
    }
}

impl<Func, T0> System for FnSystem<(T0,), Func>
where
    for<'a, 'b> &'a mut Func: FnMut(T0) + FnMut(T0::Item<'b>),
    T0: SystemArg,
{
    fn run(&mut self, map: &mut ComponentMap) {
        let p0 = T0::fetch(map);
        call_func(&mut self.func, p0);

        fn call_func<T0, F>(mut func: F, p0: T0)
        where
            F: FnMut(T0),
        {
            func(p0)
        }
    }
}

impl<Func, T0, T1> System for FnSystem<(T0, T1), Func>
where
    for<'a, 'b> &'a mut Func: FnMut(T0, T1) + FnMut(T0::Item<'b>, T1::Item<'b>),
    T0: SystemArg,
    T1: SystemArg,
{
    fn run(&mut self, map: &mut ComponentMap) {
        let p0 = T0::fetch(map);
        let p1 = T1::fetch(map);
        call_func(&mut self.func, p0, p1);

        fn call_func<T0, T1, F>(mut func: F, p0: T0, p1: T1)
        where
            F: FnMut(T0, T1),
        {
            func(p0, p1)
        }
    }
}

impl<Func, T0, T1, T2> System for FnSystem<(T0, T1, T2), Func>
where
    for<'a, 'b> &'a mut Func: FnMut(T0, T1, T2) + FnMut(T0::Item<'b>, T1::Item<'b>, T2::Item<'b>),
    T0: SystemArg,
    T1: SystemArg,
    T2: SystemArg,
{
    fn run(&mut self, map: &mut ComponentMap) {
        let p0 = T0::fetch(map);
        let p1 = T1::fetch(map);
        let p2 = T2::fetch(map);
        call_func(&mut self.func, p0, p1, p2);

        fn call_func<T0, T1, T2, F>(mut func: F, p0: T0, p1: T1, p2: T2)
        where
            F: FnMut(T0, T1, T2),
        {
            func(p0, p1, p2)
        }
    }
}

impl<Func> SystemBuilder<()> for Func
where
    for<'a> &'a mut Func: FnMut(),
    Func: FnMut(),
{
    type System = FnSystem<((),), Self>;

    fn build_system(self) -> Self::System {
        FnSystem { func: self, marker: PhantomData }
    }
}

impl<Func, T0> SystemBuilder<(T0,)> for Func
where
    for<'a, 'b> &'a mut Func: FnMut(T0) + FnMut(T0::Item<'b>),
    Func: FnMut(T0),
    T0: SystemArg,
{
    type System = FnSystem<(T0,), Self>;

    fn build_system(self) -> Self::System {
        FnSystem { func: self, marker: PhantomData }
    }
}

impl<Func, T0, T1> SystemBuilder<(T0, T1)> for Func
where
    for<'a, 'b> &'a mut Func: FnMut(T0, T1) + FnMut(T0::Item<'b>, T1::Item<'b>),
    Func: FnMut(T0, T1),
    T0: SystemArg,
    T1: SystemArg,
{
    type System = FnSystem<(T0, T1), Self>;

    fn build_system(self) -> Self::System {
        FnSystem { func: self, marker: PhantomData }
    }
}

impl<Func, T0, T1, T2> SystemBuilder<(T0, T1, T2)> for Func
where
    for<'a, 'b> &'a mut Func: FnMut(T0, T1, T2) + FnMut(T0::Item<'b>, T1::Item<'b>, T2::Item<'b>),
    Func: FnMut(T0, T1, T2),
    T0: SystemArg,
    T1: SystemArg,
    T2: SystemArg,
{
    type System = FnSystem<(T0, T1, T2), Self>;

    fn build_system(self) -> Self::System {
        FnSystem { func: self, marker: PhantomData }
    }
}

pub struct Query<'d, T> {
    inner: Vec<Ref<'d, Box<dyn Any>>>,
    marker: PhantomData<&'d T>,
}

impl<'d, T> IntoIterator for &'d Query<'d, T>
where
    T: 'static,
{
    type Item = &'d T;

    type IntoIter = std::vec::IntoIter<&'d T>;

    #[rustfmt::skip]
    fn into_iter(self) -> Self::IntoIter {
        self.inner
            .iter()
            .filter_map(|val| (val).downcast_ref())
            .collect::<Vec<&'d T>>()
            .into_iter()
    }
}

pub struct QueryMut<'d, T> {
    inner: Vec<RefMut<'d, Box<dyn Any>>>,
    marker: PhantomData<&'d mut T>,
}

pub struct QueryIterMut<'d, T> {
    inner: std::vec::IntoIter<RefMut<'d, Box<dyn Any>>>,
    marker: PhantomData<&'d mut T>,
}

impl<'d, T> IntoIterator for QueryMut<'d, T>
where
    T: 'static,
{
    type Item = RefMut<'d, T>;

    type IntoIter = QueryIterMut<'d, T>;

    fn into_iter(self) -> Self::IntoIter {
        QueryIterMut { inner: self.inner.into_iter(), marker: PhantomData }
    }
}

impl<'d, T> Iterator for QueryIterMut<'d, T>
where
    T: 'static,
{
    type Item = RefMut<'d, T>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'d, T> SystemArg for Query<'d, T>
where
    T: 'static,
{
    type Item<'o> = Query<'o, T>;

    fn fetch<'i>(components: &'i ComponentMap) -> Self::Item<'i> {
        components.query_components()
    }
}

impl<'d, T> SystemArg for QueryMut<'d, T>
where
    T: 'static,
{
    type Item<'o> = QueryMut<'o, T>;

    fn fetch<'i>(components: &'i ComponentMap) -> Self::Item<'i> {
        components.query_components_mut()
    }
}

#[cfg(test)]
mod tests {}
