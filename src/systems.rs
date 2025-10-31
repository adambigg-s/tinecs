use std::marker::PhantomData;

use crate::{ComponentMap, System};

pub trait SystemBuilder<In> {
    type System: System;

    fn build_system(self) -> Self::System;
}

pub struct FnSystem<In, Func> {
    func: Func,
    marker: PhantomData<fn() -> In>,
}

pub(crate) trait SystemArg {
    type Item<'o>;

    fn fetch<'i>(components: &'i ComponentMap) -> Self::Item<'i>;
}

impl<Func> System for FnSystem<((),), Func>
where
    for<'a> &'a mut Func: FnMut(),
    Func: Send + Sync,
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
    Func: Send + Sync,
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
    Func: Send + Sync,
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
    Func: Send + Sync,
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

impl<Func, T0, T1, T2, T3> System for FnSystem<(T0, T1, T2, T3), Func>
where
    for<'a, 'b> &'a mut Func:
        FnMut(T0, T1, T2, T3) + FnMut(T0::Item<'b>, T1::Item<'b>, T2::Item<'b>, T3::Item<'b>),
    T0: SystemArg,
    T1: SystemArg,
    T2: SystemArg,
    T3: SystemArg,
    Func: Send + Sync,
{
    fn run(&mut self, map: &mut ComponentMap) {
        let p0 = T0::fetch(map);
        let p1 = T1::fetch(map);
        let p2 = T2::fetch(map);
        let p3 = T3::fetch(map);
        call_func(&mut self.func, p0, p1, p2, p3);

        fn call_func<T0, T1, T2, T3, F>(mut func: F, p0: T0, p1: T1, p2: T2, p3: T3)
        where
            F: FnMut(T0, T1, T2, T3),
        {
            func(p0, p1, p2, p3)
        }
    }
}

impl<Func, T0, T1, T2, T3, T4> System for FnSystem<(T0, T1, T2, T3, T4), Func>
where
    for<'a, 'b> &'a mut Func: FnMut(T0, T1, T2, T3, T4)
        + FnMut(T0::Item<'b>, T1::Item<'b>, T2::Item<'b>, T3::Item<'b>, T4::Item<'b>),
    T0: SystemArg,
    T1: SystemArg,
    T2: SystemArg,
    T3: SystemArg,
    T4: SystemArg,
    Func: Send + Sync,
{
    fn run(&mut self, map: &mut ComponentMap) {
        let p0 = T0::fetch(map);
        let p1 = T1::fetch(map);
        let p2 = T2::fetch(map);
        let p3 = T3::fetch(map);
        let p4 = T4::fetch(map);
        call_func(&mut self.func, p0, p1, p2, p3, p4);

        fn call_func<T0, T1, T2, T3, T4, F>(mut func: F, p0: T0, p1: T1, p2: T2, p3: T3, p4: T4)
        where
            F: FnMut(T0, T1, T2, T3, T4),
        {
            func(p0, p1, p2, p3, p4)
        }
    }
}

impl<Func, T0, T1, T2, T3, T4, T5> System for FnSystem<(T0, T1, T2, T3, T4, T5), Func>
where
    for<'a, 'b> &'a mut Func: FnMut(T0, T1, T2, T3, T4, T5)
        + FnMut(T0::Item<'b>, T1::Item<'b>, T2::Item<'b>, T3::Item<'b>, T4::Item<'b>, T5::Item<'b>),
    T0: SystemArg,
    T1: SystemArg,
    T2: SystemArg,
    T3: SystemArg,
    T4: SystemArg,
    T5: SystemArg,
    Func: Send + Sync,
{
    fn run(&mut self, map: &mut ComponentMap) {
        let p0 = T0::fetch(map);
        let p1 = T1::fetch(map);
        let p2 = T2::fetch(map);
        let p3 = T3::fetch(map);
        let p4 = T4::fetch(map);
        let p5 = T5::fetch(map);
        call_func(&mut self.func, p0, p1, p2, p3, p4, p5);

        fn call_func<T0, T1, T2, T3, T4, T5, F>(mut func: F, p0: T0, p1: T1, p2: T2, p3: T3, p4: T4, p5: T5)
        where
            F: FnMut(T0, T1, T2, T3, T4, T5),
        {
            func(p0, p1, p2, p3, p4, p5)
        }
    }
}

impl<Func, T0, T1, T2, T3, T4, T5, T6> System for FnSystem<(T0, T1, T2, T3, T4, T5, T6), Func>
where
    for<'a, 'b> &'a mut Func: FnMut(T0, T1, T2, T3, T4, T5, T6)
        + FnMut(
            T0::Item<'b>,
            T1::Item<'b>,
            T2::Item<'b>,
            T3::Item<'b>,
            T4::Item<'b>,
            T5::Item<'b>,
            T6::Item<'b>,
        ),
    T0: SystemArg,
    T1: SystemArg,
    T2: SystemArg,
    T3: SystemArg,
    T4: SystemArg,
    T5: SystemArg,
    T6: SystemArg,
    Func: Send + Sync,
{
    #[allow(clippy::too_many_arguments)]
    fn run(&mut self, map: &mut ComponentMap) {
        let p0 = T0::fetch(map);
        let p1 = T1::fetch(map);
        let p2 = T2::fetch(map);
        let p3 = T3::fetch(map);
        let p4 = T4::fetch(map);
        let p5 = T5::fetch(map);
        let p6 = T6::fetch(map);
        call_func(&mut self.func, p0, p1, p2, p3, p4, p5, p6);

        fn call_func<T0, T1, T2, T3, T4, T5, T6, F>(
            mut func: F,
            p0: T0,
            p1: T1,
            p2: T2,
            p3: T3,
            p4: T4,
            p5: T5,
            p6: T6,
        ) where
            F: FnMut(T0, T1, T2, T3, T4, T5, T6),
        {
            func(p0, p1, p2, p3, p4, p5, p6)
        }
    }
}

impl<Func, T0, T1, T2, T3, T4, T5, T6, T7> System for FnSystem<(T0, T1, T2, T3, T4, T5, T6, T7), Func>
where
    for<'a, 'b> &'a mut Func: FnMut(T0, T1, T2, T3, T4, T5, T6, T7)
        + FnMut(
            T0::Item<'b>,
            T1::Item<'b>,
            T2::Item<'b>,
            T3::Item<'b>,
            T4::Item<'b>,
            T5::Item<'b>,
            T6::Item<'b>,
            T7::Item<'b>,
        ),
    T0: SystemArg,
    T1: SystemArg,
    T2: SystemArg,
    T3: SystemArg,
    T4: SystemArg,
    T5: SystemArg,
    T6: SystemArg,
    T7: SystemArg,
    Func: Send + Sync,
{
    #[allow(clippy::too_many_arguments)]
    fn run(&mut self, map: &mut ComponentMap) {
        let p0 = T0::fetch(map);
        let p1 = T1::fetch(map);
        let p2 = T2::fetch(map);
        let p3 = T3::fetch(map);
        let p4 = T4::fetch(map);
        let p5 = T5::fetch(map);
        let p6 = T6::fetch(map);
        let p7 = T7::fetch(map);
        call_func(&mut self.func, p0, p1, p2, p3, p4, p5, p6, p7);

        fn call_func<T0, T1, T2, T3, T4, T5, T6, T7, F>(
            mut func: F,
            p0: T0,
            p1: T1,
            p2: T2,
            p3: T3,
            p4: T4,
            p5: T5,
            p6: T6,
            p7: T7,
        ) where
            F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7),
        {
            func(p0, p1, p2, p3, p4, p5, p6, p7)
        }
    }
}

impl<Func, T0, T1, T2, T3, T4, T5, T6, T7, T8> System for FnSystem<(T0, T1, T2, T3, T4, T5, T6, T7, T8), Func>
where
    for<'a, 'b> &'a mut Func: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8)
        + FnMut(
            T0::Item<'b>,
            T1::Item<'b>,
            T2::Item<'b>,
            T3::Item<'b>,
            T4::Item<'b>,
            T5::Item<'b>,
            T6::Item<'b>,
            T7::Item<'b>,
            T8::Item<'b>,
        ),
    T0: SystemArg,
    T1: SystemArg,
    T2: SystemArg,
    T3: SystemArg,
    T4: SystemArg,
    T5: SystemArg,
    T6: SystemArg,
    T7: SystemArg,
    T8: SystemArg,
    Func: Send + Sync,
{
    #[allow(clippy::too_many_arguments)]
    fn run(&mut self, map: &mut ComponentMap) {
        let p0 = T0::fetch(map);
        let p1 = T1::fetch(map);
        let p2 = T2::fetch(map);
        let p3 = T3::fetch(map);
        let p4 = T4::fetch(map);
        let p5 = T5::fetch(map);
        let p6 = T6::fetch(map);
        let p7 = T7::fetch(map);
        let p8 = T8::fetch(map);
        call_func(&mut self.func, p0, p1, p2, p3, p4, p5, p6, p7, p8);

        fn call_func<T0, T1, T2, T3, T4, T5, T6, T7, T8, F>(
            mut func: F,
            p0: T0,
            p1: T1,
            p2: T2,
            p3: T3,
            p4: T4,
            p5: T5,
            p6: T6,
            p7: T7,
            p8: T8,
        ) where
            F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8),
        {
            func(p0, p1, p2, p3, p4, p5, p6, p7, p8)
        }
    }
}

impl<Func, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> System
    for FnSystem<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), Func>
where
    for<'a, 'b> &'a mut Func: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
        + FnMut(
            T0::Item<'b>,
            T1::Item<'b>,
            T2::Item<'b>,
            T3::Item<'b>,
            T4::Item<'b>,
            T5::Item<'b>,
            T6::Item<'b>,
            T7::Item<'b>,
            T8::Item<'b>,
            T9::Item<'b>,
        ),
    T0: SystemArg,
    T1: SystemArg,
    T2: SystemArg,
    T3: SystemArg,
    T4: SystemArg,
    T5: SystemArg,
    T6: SystemArg,
    T7: SystemArg,
    T8: SystemArg,
    T9: SystemArg,
    Func: Send + Sync,
{
    #[allow(clippy::too_many_arguments)]
    fn run(&mut self, map: &mut ComponentMap) {
        let p0 = T0::fetch(map);
        let p1 = T1::fetch(map);
        let p2 = T2::fetch(map);
        let p3 = T3::fetch(map);
        let p4 = T4::fetch(map);
        let p5 = T5::fetch(map);
        let p6 = T6::fetch(map);
        let p7 = T7::fetch(map);
        let p8 = T8::fetch(map);
        let p9 = T9::fetch(map);
        call_func(&mut self.func, p0, p1, p2, p3, p4, p5, p6, p7, p8, p9);

        fn call_func<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, F>(
            mut func: F,
            p0: T0,
            p1: T1,
            p2: T2,
            p3: T3,
            p4: T4,
            p5: T5,
            p6: T6,
            p7: T7,
            p8: T8,
            p9: T9,
        ) where
            F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9),
        {
            func(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9)
        }
    }
}

impl<Func> SystemBuilder<()> for Func
where
    for<'a> &'a mut Func: FnMut(),
    Func: FnMut() + Send + Sync,
{
    type System = FnSystem<((),), Self>;

    fn build_system(self) -> Self::System {
        FnSystem { func: self, marker: PhantomData }
    }
}

impl<Func, T0> SystemBuilder<(T0,)> for Func
where
    for<'a, 'b> &'a mut Func: FnMut(T0) + FnMut(T0::Item<'b>),
    Func: FnMut(T0) + Send + Sync,
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
    Func: FnMut(T0, T1) + Send + Sync,
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
    Func: FnMut(T0, T1, T2) + Send + Sync,
    T0: SystemArg,
    T1: SystemArg,
    T2: SystemArg,
{
    type System = FnSystem<(T0, T1, T2), Self>;

    fn build_system(self) -> Self::System {
        FnSystem { func: self, marker: PhantomData }
    }
}

impl<Func, T0, T1, T2, T3> SystemBuilder<(T0, T1, T2, T3)> for Func
where
    for<'a, 'b> &'a mut Func:
        FnMut(T0, T1, T2, T3) + FnMut(T0::Item<'b>, T1::Item<'b>, T2::Item<'b>, T3::Item<'b>),
    Func: FnMut(T0, T1, T2, T3) + Send + Sync,
    T0: SystemArg,
    T1: SystemArg,
    T2: SystemArg,
    T3: SystemArg,
{
    type System = FnSystem<(T0, T1, T2, T3), Self>;

    fn build_system(self) -> Self::System {
        FnSystem { func: self, marker: PhantomData }
    }
}

impl<Func, T0, T1, T2, T3, T4> SystemBuilder<(T0, T1, T2, T3, T4)> for Func
where
    for<'a, 'b> &'a mut Func: FnMut(T0, T1, T2, T3, T4)
        + FnMut(T0::Item<'b>, T1::Item<'b>, T2::Item<'b>, T3::Item<'b>, T4::Item<'b>),
    Func: FnMut(T0, T1, T2, T3, T4) + Send + Sync,
    T0: SystemArg,
    T1: SystemArg,
    T2: SystemArg,
    T3: SystemArg,
    T4: SystemArg,
{
    type System = FnSystem<(T0, T1, T2, T3, T4), Self>;

    fn build_system(self) -> Self::System {
        FnSystem { func: self, marker: PhantomData }
    }
}

impl<Func, T0, T1, T2, T3, T4, T5> SystemBuilder<(T0, T1, T2, T3, T4, T5)> for Func
where
    for<'a, 'b> &'a mut Func: FnMut(T0, T1, T2, T3, T4, T5)
        + FnMut(T0::Item<'b>, T1::Item<'b>, T2::Item<'b>, T3::Item<'b>, T4::Item<'b>, T5::Item<'b>),
    Func: FnMut(T0, T1, T2, T3, T4, T5) + Send + Sync,
    T0: SystemArg,
    T1: SystemArg,
    T2: SystemArg,
    T3: SystemArg,
    T4: SystemArg,
    T5: SystemArg,
{
    type System = FnSystem<(T0, T1, T2, T3, T4, T5), Self>;

    fn build_system(self) -> Self::System {
        FnSystem { func: self, marker: PhantomData }
    }
}

impl<Func, T0, T1, T2, T3, T4, T5, T6> SystemBuilder<(T0, T1, T2, T3, T4, T5, T6)> for Func
where
    for<'a, 'b> &'a mut Func: FnMut(T0, T1, T2, T3, T4, T5, T6)
        + FnMut(
            T0::Item<'b>,
            T1::Item<'b>,
            T2::Item<'b>,
            T3::Item<'b>,
            T4::Item<'b>,
            T5::Item<'b>,
            T6::Item<'b>,
        ),
    Func: FnMut(T0, T1, T2, T3, T4, T5, T6) + Send + Sync,
    T0: SystemArg,
    T1: SystemArg,
    T2: SystemArg,
    T3: SystemArg,
    T4: SystemArg,
    T5: SystemArg,
    T6: SystemArg,
{
    type System = FnSystem<(T0, T1, T2, T3, T4, T5, T6), Self>;

    fn build_system(self) -> Self::System {
        FnSystem { func: self, marker: PhantomData }
    }
}

impl<Func, T0, T1, T2, T3, T4, T5, T6, T7> SystemBuilder<(T0, T1, T2, T3, T4, T5, T6, T7)> for Func
where
    for<'a, 'b> &'a mut Func: FnMut(T0, T1, T2, T3, T4, T5, T6, T7)
        + FnMut(
            T0::Item<'b>,
            T1::Item<'b>,
            T2::Item<'b>,
            T3::Item<'b>,
            T4::Item<'b>,
            T5::Item<'b>,
            T6::Item<'b>,
            T7::Item<'b>,
        ),
    Func: FnMut(T0, T1, T2, T3, T4, T5, T6, T7) + Send + Sync,
    T0: SystemArg,
    T1: SystemArg,
    T2: SystemArg,
    T3: SystemArg,
    T4: SystemArg,
    T5: SystemArg,
    T6: SystemArg,
    T7: SystemArg,
{
    type System = FnSystem<(T0, T1, T2, T3, T4, T5, T6, T7), Self>;

    fn build_system(self) -> Self::System {
        FnSystem { func: self, marker: PhantomData }
    }
}

impl<Func, T0, T1, T2, T3, T4, T5, T6, T7, T8> SystemBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8)> for Func
where
    for<'a, 'b> &'a mut Func: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8)
        + FnMut(
            T0::Item<'b>,
            T1::Item<'b>,
            T2::Item<'b>,
            T3::Item<'b>,
            T4::Item<'b>,
            T5::Item<'b>,
            T6::Item<'b>,
            T7::Item<'b>,
            T8::Item<'b>,
        ),
    Func: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8) + Send + Sync,
    T0: SystemArg,
    T1: SystemArg,
    T2: SystemArg,
    T3: SystemArg,
    T4: SystemArg,
    T5: SystemArg,
    T6: SystemArg,
    T7: SystemArg,
    T8: SystemArg,
{
    type System = FnSystem<(T0, T1, T2, T3, T4, T5, T6, T7, T8), Self>;

    fn build_system(self) -> Self::System {
        FnSystem { func: self, marker: PhantomData }
    }
}

impl<Func, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> SystemBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)>
    for Func
where
    for<'a, 'b> &'a mut Func: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
        + FnMut(
            T0::Item<'b>,
            T1::Item<'b>,
            T2::Item<'b>,
            T3::Item<'b>,
            T4::Item<'b>,
            T5::Item<'b>,
            T6::Item<'b>,
            T7::Item<'b>,
            T8::Item<'b>,
            T9::Item<'b>,
        ),
    Func: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) + Send + Sync,
    T0: SystemArg,
    T1: SystemArg,
    T2: SystemArg,
    T3: SystemArg,
    T4: SystemArg,
    T5: SystemArg,
    T6: SystemArg,
    T7: SystemArg,
    T8: SystemArg,
    T9: SystemArg,
{
    type System = FnSystem<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), Self>;

    fn build_system(self) -> Self::System {
        FnSystem { func: self, marker: PhantomData }
    }
}
