use tinecs::{Component, Master, arguments::QueryMut};

fn main() {
    const STEPS: usize = 10_000_000;

    let mut fib = Box::new(Fib::new());
    let start = std::time::Instant::now();
    for _ in 1..STEPS {
        fib_step_standard(&mut fib);
    }
    println!("standard fib time: {}\nfib number: {}", start.elapsed().as_secs_f64(), fib.num);

    let mut ecs = Master::default();
    let fib = ecs.create_entity();
    ecs.add_component(fib, Fib::new());
    ecs.add_system(fib_step_ecs);
    let start = std::time::Instant::now();
    for _ in 1..STEPS {
        ecs.run();
    }
    println!(
        "tiny-ecs fib time: {}\nfib number: {}",
        start.elapsed().as_secs_f64(),
        ecs.query::<Fib>().make_singular().num
    );
}

#[inline(never)]
fn fib_step_standard(fib: &mut Fib) {
    fib.step();
}

#[inline(never)]
fn fib_step_ecs(fib: QueryMut<Fib>) {
    for mut fib in fib {
        fib.step();
    }
}

impl Component for Fib {}
struct Fib {
    prev: usize,
    num: usize,
}

impl Fib {
    pub fn new() -> Self {
        Self { prev: 0, num: 1 }
    }

    #[inline(never)]
    pub fn step(&mut self) {
        (self.num, self.prev) = (self.num.wrapping_add(self.prev), self.num);
    }
}
