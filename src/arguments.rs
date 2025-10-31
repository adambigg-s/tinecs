use std::{
    any::Any,
    cell::{Ref, RefMut},
    marker::PhantomData,
};

use crate::{Component, ComponentMap, RepresentAsAny, systems::SystemArg};

pub struct Query<'d, T> {
    pub(crate) inner: Vec<Ref<'d, Box<dyn Component>>>,
    pub(crate) marker: PhantomData<&'d T>,
}

pub struct QueryIter<'d, T> {
    inner: std::vec::IntoIter<&'d T>,
    marker: PhantomData<&'d T>,
}

impl<'d, T> IntoIterator for &'d Query<'d, T>
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
                .filter_map(|val| (val).as_any().downcast_ref())
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

impl<'d, T> SystemArg for Query<'d, T>
where
    T: 'static,
{
    type Item<'o> = Query<'o, T>;

    fn fetch<'i>(components: &'i ComponentMap) -> Self::Item<'i> {
        components.query_components()
    }
}

pub struct QueryMut<'d, T> {
    pub(crate) inner: Vec<RefMut<'d, Box<dyn Component>>>,
    pub(crate) marker: PhantomData<&'d mut T>,
}

pub struct QueryIterMut<'d, T> {
    inner: std::vec::IntoIter<RefMut<'d, Box<dyn Component>>>,
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
        Some(RefMut::map(self.inner.next()?, |val| val.as_any_mut().downcast_mut().unwrap()))
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
