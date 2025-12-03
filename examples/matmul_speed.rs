use nalgebra::Matrix4;
use tinecs::{Component, Master, arguments::QueryMut};

fn main() {
    const STEPS: usize = 100_000_000;

    let mut similarity_transformation = Box::new(Conjugation::new());
    let start = std::time::Instant::now();
    for _ in 0..STEPS {
        conjugate_standard(&mut similarity_transformation);
    }
    println!(
        "standard similarity transformation time: {}\nmatrix: {}",
        start.elapsed().as_secs_f64(),
        similarity_transformation.mat_a
    );

    let mut ecs = Master::default();
    let similarity_transformation = ecs.create_entity();
    ecs.add_component(similarity_transformation, Conjugation::new());
    ecs.add_system(conjugate_ecs);
    let start = std::time::Instant::now();
    for _ in 0..STEPS {
        ecs.run();
    }
    println!(
        "tiny-ecs similarity transformation time: {}\nmatrix: {}",
        start.elapsed().as_secs_f64(),
        ecs.query::<Conjugation>().make_singular().mat_a
    );
}

#[inline(never)]
fn conjugate_standard(matrices: &mut Conjugation) {
    matrices.transform();
}

#[inline(never)]
fn conjugate_ecs(matrices: QueryMut<Conjugation>) {
    for mut matrices in matrices {
        matrices.transform();
    }
}

impl Component for Conjugation {}
struct Conjugation {
    mat_p: Matrix4<f64>,
    mat_a: Matrix4<f64>,
}

impl Conjugation {
    pub fn new() -> Self {
        Self {
            mat_p: Matrix4::from_euler_angles(0.1, 0.2, 0.3),
            mat_a: Matrix4::identity(),
        }
    }

    #[inline(never)]
    pub fn transform(&mut self) {
        self.mat_a = self.mat_p.transpose() * self.mat_a * self.mat_p;
    }
}
