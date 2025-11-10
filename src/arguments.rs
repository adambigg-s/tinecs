use std::{
    any::Any,
    cell::{Ref, RefMut},
    marker::PhantomData,
};

use crate::{Component, ComponentMap, Entity, systems::SystemArg};

pub trait QueryFilter {
    fn matches(components: &ComponentMap, entity: Entity) -> bool;
}

impl QueryFilter for () {
    fn matches(_: &ComponentMap, _: Entity) -> bool {
        true
    }
}

pub struct With<T>(PhantomData<T>);

impl<T> QueryFilter for With<T>
where
    T: 'static,
{
    fn matches(components: &ComponentMap, entity: Entity) -> bool {
        components.has_component::<T>(entity)
    }
}

pub struct Without<T>(PhantomData<T>);

impl<T> QueryFilter for Without<T>
where
    T: 'static,
{
    fn matches(components: &ComponentMap, entity: Entity) -> bool {
        !components.has_component::<T>(entity)
    }
}

pub(crate) struct InnerCluster<'d> {
    pub(crate) entity: Entity,
    pub(crate) component: Ref<'d, Box<dyn Component>>,
}

pub struct Query<'d, T, F = ()> {
    pub(crate) inner: Vec<InnerCluster<'d>>,
    pub(crate) marker: PhantomData<&'d T>,
    pub(crate) fmarker: PhantomData<F>,
}

pub struct QueryIter<'d, T> {
    inner: std::vec::IntoIter<&'d T>,
    marker: PhantomData<&'d T>,
}

impl<'d, T, F> IntoIterator for &'d Query<'d, T, F>
where
    T: Component + Any + 'static,
{
    type Item = &'d T;

    type IntoIter = QueryIter<'d, T>;

    fn into_iter(self) -> Self::IntoIter {
        QueryIter {
            inner: self
                .inner
                .iter()
                .filter_map(|cluster| cluster.component.as_any().downcast_ref())
                .collect::<Vec<&'d T>>()
                .into_iter(),
            marker: PhantomData,
        }
    }
}

impl<'d, T> Iterator for QueryIter<'d, T> {
    type Item = &'d T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'d, T, F> SystemArg for Query<'d, T, F>
where
    T: 'static,
    F: QueryFilter,
{
    type Item<'o> = Query<'o, T, F>;

    fn fetch<'i>(components: &'i ComponentMap) -> Self::Item<'i> {
        components.query_components_filtered()
    }
}

pub(crate) struct InnerClusterMut<'d> {
    pub(crate) entity: Entity,
    pub(crate) component: RefMut<'d, Box<dyn Component>>,
}

pub struct QueryMut<'d, T, F = ()> {
    pub(crate) inner: Vec<InnerClusterMut<'d>>,
    pub(crate) marker: PhantomData<&'d mut T>,
    pub(crate) fmarker: PhantomData<F>,
}

pub struct QueryIterMut<'d, T> {
    inner: std::vec::IntoIter<InnerClusterMut<'d>>,
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
        Some(RefMut::map(self.inner.next()?.component, |value| {
            value.as_any_mut().downcast_mut().unwrap()
        }))
    }
}

impl<'d, T> SystemArg for QueryMut<'d, T>
where
    T: 'static,
{
    type Item<'o> = QueryMut<'o, T>;

    fn fetch<'i>(components: &'i ComponentMap) -> Self::Item<'i> {
        components.query_components_mut_filtered()
    }
}
