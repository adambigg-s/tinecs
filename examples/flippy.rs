use euc::{Buffer2d, IndexedVertices, Pipeline, Target, TriangleList};
use minifb::{Key, Window, WindowOptions};
use nalgebra::{Matrix3, Matrix4, SVector, Vector3, Vector4};
use tinecs::{
    Component,
    arguments::{Query, QueryMut, With},
    master,
};

fn main() {
    let [width, height] = [800, 600];
    let mut window = Window::new("flippy floppy", width, height, WindowOptions::default()).unwrap();
    let mut ecs = master();

    #[rustfmt::skip]
    let dcm = DirectionCosine(Matrix3::from_row_slice(&[
        1.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 0.0, 1.0,
    ]));
    #[rustfmt::skip]
    let moi = InertialTensor(Matrix3::from_row_slice(&[
        5.0, 0.0, 0.0,
        0.0, 3.0, 0.0,
        0.0, 0.0, 1.0,
    ]));
    #[rustfmt::skip]
    let vel = AngularVelocity(Vector3::from_row_slice(&[
        0.01, 1.0, 0.001,
    ]));
    let mut state = SVector::<f32, 21>::zeros();
    state.rows_mut(0, 9).copy_from_slice(dcm.0.as_slice());
    state.rows_mut(9, 9).copy_from_slice(moi.0.as_slice());
    state.rows_mut(18, 3).copy_from_slice(vel.0.as_slice());
    let integrator = Integrator {
        state,
        t: f32::default(),
        dt: 0.01,
        ddt: flippy_dymamics,
        tolerance: 1e-6,
    };

    let render_target = RenderTarget {
        color: Buffer2d::fill([width, height], u32::default()),
        depth: Buffer2d::fill([width, height], f32::MAX),
    };
    let camera = Camera {
        view: Matrix4::new_translation(&nalgebra::Vector3::new(0.0, 0.0, -10.0)),
        projection: Matrix4::new_perspective(
            (width as f32) / (height as f32),
            90.0_f32.to_radians(),
            0.1,
            100.0,
        ),
    };
    let mesh = Renderable { vertices: VERTICES, indices: INDICES };

    let object = ecs.create_entity();
    let renderer = ecs.create_entity();
    let viewer = ecs.create_entity();
    ecs.add_component(object, moi);
    ecs.add_component(object, vel);
    ecs.add_component(object, dcm);
    ecs.add_component(object, mesh);
    ecs.add_component(object, integrator);
    ecs.add_component(object, DynamicBody);
    ecs.add_component(renderer, render_target);
    ecs.add_component(viewer, camera);
    ecs.add_system(integrate_body::<21>);
    ecs.add_system(render_frame);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for RenderTarget { color, .. } in &ecs.query::<RenderTarget>() {
            _ = window.update_with_buffer(color.raw(), width, height);
        }

        ecs.run();
    }
}

fn render_frame(
    camera: Query<Camera>,
    mesh: Query<Renderable>,
    object: Query<DirectionCosine, With<DynamicBody>>,
    target: QueryMut<RenderTarget>,
) {
    let camera = camera.make_singular();
    let mesh = mesh.make_singular();

    for mut target in target {
        target.color.clear(u32::default());
        target.depth.clear(f32::MAX);
        let (color, depth) = target.split_mut();
        for DirectionCosine(object) in &object {
            Cube {
                transform: camera.projection * camera.view * object.to_homogeneous(),
            }
            .render(IndexedVertices::new(mesh.indices, mesh.vertices), color, depth);
        }
    }
}

impl Component for Renderable {}
struct Renderable {
    vertices: &'static [(Vector4<f32>, Vector4<f32>)],
    indices: &'static [usize],
}

impl Component for Camera {}
struct Camera {
    view: Matrix4<f32>,
    projection: Matrix4<f32>,
}

impl Component for RenderTarget {}
struct RenderTarget {
    color: Buffer2d<u32>,
    depth: Buffer2d<f32>,
}

impl RenderTarget {
    fn split_mut(&mut self) -> (&mut Buffer2d<u32>, &mut Buffer2d<f32>) {
        (&mut self.color, &mut self.depth)
    }
}

impl Component for DirectionCosine {}
struct DirectionCosine(Matrix3<f32>);

impl Component for AngularVelocity {}
struct AngularVelocity(Vector3<f32>);

impl Component for InertialTensor {}
struct InertialTensor(Matrix3<f32>);

impl Component for DynamicBody {}
struct DynamicBody;

struct Cube {
    transform: Matrix4<f32>,
}

impl<'d> Pipeline<'d> for Cube {
    type Vertex = (Vector4<f32>, Vector4<f32>);

    type VertexData = Vector4<f32>;

    type Primitives = TriangleList;

    type Fragment = Vector4<f32>;

    type Pixel = u32;

    fn vertex(&self, (pos, color): &Self::Vertex) -> ([f32; 4], Self::VertexData) {
        ((self.transform * pos).into(), *color)
    }

    fn fragment(&self, color: Self::VertexData) -> Self::Fragment {
        color
    }

    fn blend(&self, _: Self::Pixel, color: Self::Fragment) -> Self::Pixel {
        let r = (color.x * u8::MAX as f32).clamp(0.0, u8::MAX as f32) as u32;
        let g = (color.y * u8::MAX as f32).clamp(0.0, u8::MAX as f32) as u32;
        let b = (color.z * u8::MAX as f32).clamp(0.0, u8::MAX as f32) as u32;
        (r << 16) | (g << 8) | b
    }
}

fn integrate_body<const N: usize>(
    integrator: QueryMut<Integrator<N>, With<DynamicBody>>,
    dcm: QueryMut<DirectionCosine, With<DynamicBody>>,
    vel: QueryMut<AngularVelocity, With<DynamicBody>>,
) {
    let mut state = SVector::zeros();
    for mut integrator in integrator {
        integrator.step();
        state = integrator.state;
    }

    for mut dcm in dcm {
        *dcm = DirectionCosine(Matrix3::from_column_slice(state.rows(0, 9).as_slice()));
    }
    for mut vel in vel {
        *vel = AngularVelocity(Vector3::from_column_slice(state.rows(18, 3).as_slice()));
    }
}

fn flippy_dymamics<const N: usize>(input: &SVector<f32, N>) -> SVector<f32, N> {
    let c = Matrix3::from_column_slice(input.rows(0, 9).as_slice());
    let ic = Matrix3::from_column_slice(input.rows(9, 9).as_slice());
    let w = Vector3::from_column_slice(input.rows(18, 3).as_slice());
    let [i11, i22, i33] = [ic[(0, 0)], ic[(1, 1)], ic[(2, 2)]];
    let [w1, w2, w3] = [w[0], w[1], w[2]];

    #[rustfmt::skip]
    let s = Matrix3::from_row_slice(&[
        0.0 , -w.z, w.y ,
        w.z , 0.0 , -w.x,
        -w.y, w.x , 0.0 ,
    ]);
    let dcdt = s * c;
    #[rustfmt::skip]
    let dwdt = Vector3::new(
        (i33 - i22) * w2 * w3 / i11,
        (i11 - i33) * w3 * w1 / i22,
        (i22 - i11) * w1 * w2 / i33,
    );
    let dicdt = Matrix3::<f32>::zeros();

    let mut output = SVector::<f32, N>::zeros();
    output.rows_mut(0, 9).copy_from_slice(dcdt.as_slice());
    output.rows_mut(9, 9).copy_from_slice(dicdt.as_slice());
    output.rows_mut(18, 3).copy_from_slice(dwdt.as_slice());
    output
}

impl<const N: usize> Component for Integrator<N> {}
#[derive(Clone, Copy)]
struct Integrator<const N: usize> {
    state: SVector<f32, N>,
    t: f32,
    dt: f32,
    ddt: fn(&SVector<f32, N>) -> SVector<f32, N>,
    tolerance: f32,
}

impl<const N: usize> Integrator<N> {
    fn dynamic_step(&mut self) -> SVector<f32, N> {
        let mut oracle = *self;
        oracle.dt = self.dt * 0.5;
        (0..2).for_each(|_| {
            oracle.step();
        });
        let step = self.rk4();

        let error = (oracle.state - step).norm();
        if error > self.tolerance {
            self.dt *= 0.7;
            return self.dynamic_step();
        }

        self.dt *= 1.3;
        self.t += self.dt;
        self.state = step;
        self.state
    }

    fn step(&mut self) -> SVector<f32, N> {
        self.t += self.dt;
        self.state = self.rk4();
        self.state
    }

    fn rk4(&self) -> SVector<f32, N> {
        let k1 = (self.ddt)(&self.state);
        let k2 = (self.ddt)(&(self.state + k1 * (self.dt / 2.0)));
        let k3 = (self.ddt)(&(self.state + k2 * (self.dt / 2.0)));
        let k4 = (self.ddt)(&(self.state + k3 * self.dt));

        self.state + (k1 + k4) * self.dt / 6.0 + (k2 + k3) * self.dt / 3.0
    }
}

const R: Vector4<f32> = Vector4::new(1.0, 0.0, 0.0, 1.0);
const Y: Vector4<f32> = Vector4::new(1.0, 1.0, 0.0, 1.0);
const G: Vector4<f32> = Vector4::new(0.0, 1.0, 0.0, 1.0);
const B: Vector4<f32> = Vector4::new(0.0, 0.0, 1.0, 1.0);

const VERTICES: &[(Vector4<f32>, Vector4<f32>)] = &[
    (Vector4::new(-1.0, -1.0, -1.0, 1.0), R),
    (Vector4::new(-1.0, -1.0, 1.0, 1.0), Y),
    (Vector4::new(-1.0, 1.0, -1.0, 1.0), G),
    (Vector4::new(-1.0, 1.0, 1.0, 1.0), B),
    (Vector4::new(1.0, -1.0, -1.0, 1.0), B),
    (Vector4::new(1.0, -1.0, 1.0, 1.0), G),
    (Vector4::new(1.0, 1.0, -1.0, 1.0), Y),
    (Vector4::new(1.0, 1.0, 1.0, 1.0), R),
];

const INDICES: &[usize] = &[
    0, 3, 2, 0, 1, 3, 7, 4, 6, 5, 4, 7, 5, 0, 4, 1, 0, 5, 2, 7, 6, 2, 3, 7, 0, 6, 4, 0, 2, 6, 7, 1, 5, 3, 1,
    7,
];
