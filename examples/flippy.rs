use std::{
    ops::{Add, Mul},
    sync::Arc,
};

use euc::{
    Buffer2d, CullMode, DepthMode, IndexedVertices, Pipeline, Target, TriangleList,
    primitives::PrimitiveKind, rasterizer::Rasterizer,
};
use minifb::{Key, Window, WindowOptions};
use nalgebra::{Isometry3, Matrix3, Matrix4, Point3, SVector, Vector3, Vector4};
use tinecs::{
    Component,
    arguments::{Query, QueryMut, With},
    master,
};

/*
Components used
*/

impl Component for DynamicBody {}
struct DynamicBody;

impl Component for DirectionCosine {}
struct DirectionCosine(Matrix3<f32>);

impl Component for AngularVelocity {}
struct AngularVelocity(Vector3<f32>);

impl Component for InertialTensor {}
struct InertialTensor(Matrix3<f32>);

impl Component for PointLight {}
struct PointLight {
    pos: Vector3<f32>,
}

impl Component for Mesh {}
struct Mesh {
    vertices: Arc<Vec<Vertex>>,
    indices: Arc<Vec<usize>>,
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

impl<const N: usize> Component for Integrator<N> {}
#[derive(Clone, Copy)]
struct Integrator<const N: usize> {
    state: SVector<f32, N>,
    t: f32,
    dt: f32,
    ddt: fn(&SVector<f32, N>) -> SVector<f32, N>,
    tolerance: f32,
}

/*
Systems called by ECS
*/

fn integrate_body<const N: usize>(
    integrator: QueryMut<Integrator<N>, With<DynamicBody>>,
    dcm: QueryMut<DirectionCosine, With<DynamicBody>>,
    vel: QueryMut<AngularVelocity, With<DynamicBody>>,
) {
    let mut state = SVector::zeros();
    for mut integrator in integrator {
        integrator.dynamic_step();
        state = integrator.state;
    }

    for mut dcm in dcm {
        *dcm = DirectionCosine(Matrix3::from_column_slice(state.rows(0, 9).as_slice()));
    }
    for mut vel in vel {
        *vel = AngularVelocity(Vector3::from_column_slice(state.rows(18, 3).as_slice()));
    }
}

fn render_frame(
    camera: Query<Camera>,
    mesh: Query<Mesh>,
    dcm: Query<DirectionCosine>,
    light: Query<PointLight>,
    target: QueryMut<RenderTarget>,
) {
    let camera = camera.make_singular();
    let light = light.make_singular();
    for mut target in target {
        let (color, depth) = target.split_mut();
        for (Mesh { vertices, indices }, DirectionCosine(dcm)) in mesh.into_iter().zip(&dcm) {
            SolidUniform {
                model: *dcm,
                view: camera.view,
                proj: camera.projection,
                light: light.pos,
            }
            .render(IndexedVertices::new(indices.as_slice(), vertices.as_slice()), color, depth);
        }
    }
}

/*
Entry point
*/

fn main() {
    let [width, height] = [800, 600];

    physics_setup();
    graphics_setup(width, height);

    let mut window = Window::new("flippy floppy", width, height, WindowOptions::default()).unwrap();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for mut target in master().query_mut::<RenderTarget>() {
            window.update_with_buffer(target.color.raw(), width, height).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(15));
            let (color, depth) = target.split_mut();
            color.clear(u32::default());
            depth.clear(f32::MAX);
        }

        master().run();
    }
}

/*
Scene setup functions (out of main for readability)
*/

#[rustfmt::skip]
fn physics_setup() {
    let dcm = DirectionCosine(Matrix3::from_row_slice(&[
        1.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 0.0, 1.0,
    ]));
    let moi = InertialTensor(Matrix3::from_row_slice(&[
        15.0, 0.0, 0.0,
        0.0, 5.0, 0.0,
        0.0, 0.0, 4.0,
    ]));
    let vel = AngularVelocity(Vector3::from_row_slice(&[
        0.05, 3.5, 0.25,
    ]));
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut add_cuboid = |lengths, pos, color| {
        let (verts, mut inds) = make_cuboid(lengths, pos, color);
        inds.iter_mut().for_each(|idx| *idx += vertices.len());
        vertices.extend_from_slice(&verts[..]);
        indices.extend_from_slice(&inds);
    };
    add_cuboid(
        Vector3::new(1.0, 1.0, 2.0),
        Vector3::new(0.0, 0.0, 2.0),
        Vector4::new(0.7, 0.7, 0.7, 1.0),
    );
    add_cuboid(
        Vector3::new(1.0, 1.0, 2.0),
        Vector3::new(0.0, 0.0, -2.0),
        Vector4::new(0.7, 0.7, 0.7, 1.0),
    );
    add_cuboid(
        Vector3::new(1.0, 1.5, 1.0),
        Vector3::new(0.0, 2.5, 0.0),
        Vector4::new(0.7, 0.7, 0.7, 1.0),
    );
    add_cuboid(
        Vector3::new(0.85, 0.85, 0.1),
        Vector3::new(0.0, 0.0, 4.1),
        Vector4::new(1.0, 0.0, 0.0, 1.0),
    );
    add_cuboid(
        Vector3::new(0.85, 0.85, 0.1),
        Vector3::new(0.0, 0.0, -4.1),
        Vector4::new(0.0, 1.0, 0.0, 1.0),
    );
    add_cuboid(
        Vector3::new(0.85, 0.1, 0.85),
        Vector3::new(0.0, 4.1, 0.0),
        Vector4::new(0.0, 0.0, 1.0, 1.0),
    );
    let mesh = Mesh {
        vertices: Arc::new(vertices),
        indices: Arc::new(indices),
    };
    let (ground_verts, ground_indices) = make_cuboid(
        Vector3::new(10.0, 1.0, 10.0),
        Vector3::new(0.0, -1.0, 0.0),
        Vector4::new(0.5, 0.5, 0.5, 1.0),
    );
    let ground_mesh = Mesh {vertices: ground_verts.into(), indices: ground_indices.into() };
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

    let mut ecs = master();
    let object = ecs.create_entity();
    let ground = ecs.create_entity();
    ecs.add_component(object, moi);
    ecs.add_component(object, vel);
    ecs.add_component(object, dcm);
    ecs.add_component(object, mesh);
    ecs.add_component(object, integrator);
    ecs.add_component(object, DynamicBody);
    ecs.add_component(ground, ground_mesh);
    ecs.add_component(ground, DirectionCosine(Matrix3::identity()));
    ecs.add_system(integrate_body::<21>);
}

fn graphics_setup(width: usize, height: usize) {
    let render_target = RenderTarget {
        color: Buffer2d::fill([width, height], u32::default()),
        depth: Buffer2d::fill([width, height], f32::MAX),
    };
    let camera = Camera {
        view: Isometry3::look_at_rh(&Point3::new(0.0, 5.0, -12.5), &Point3::origin(), &Vector3::y())
            .to_matrix(),
        projection: Matrix4::new_perspective(
            (width as f32) / (height as f32),
            90.0_f32.to_radians(),
            0.1,
            100.0,
        ),
    };
    let point_light = PointLight { pos: Vector3::new(-3.0, 8.0, -7.0) };

    let mut ecs = master();
    let renderer = ecs.create_entity();
    let viewer = ecs.create_entity();
    let light = ecs.create_entity();
    ecs.add_component(renderer, render_target);
    ecs.add_component(viewer, camera);
    ecs.add_component(light, point_light);
    ecs.add_system(render_frame);
}

/*
Rendering stuff (not part of ECS)
*/

#[derive(Clone, Copy)]
struct Vertex {
    pos: Vector4<f32>,
    col: Vector4<f32>,
    nor: Vector3<f32>,
}

impl Add for Vertex {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            pos: self.pos + other.pos,
            col: self.col + other.col,
            nor: self.nor + other.nor,
        }
    }
}

impl Mul<f32> for Vertex {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Self {
            pos: self.pos * scalar,
            col: self.col * scalar,
            nor: self.nor * scalar,
        }
    }
}

struct SolidUniform {
    model: Matrix3<f32>,
    view: Matrix4<f32>,
    proj: Matrix4<f32>,
    light: Vector3<f32>,
}

impl<'d> Pipeline<'d> for SolidUniform {
    type Vertex = Vertex;

    type VertexData = Vertex;

    type Primitives = TriangleList;

    type Fragment = Vector4<f32>;

    type Pixel = u32;

    #[inline(always)]
    fn rasterizer_config(
        &self,
    ) -> <<Self::Primitives as PrimitiveKind<Self::VertexData>>::Rasterizer as Rasterizer>::Config {
        CullMode::None
    }

    #[inline(always)]
    fn depth_mode(&self) -> DepthMode {
        DepthMode::LESS_WRITE
    }

    #[inline(always)]
    fn vertex(&self, vertex: &Self::Vertex) -> ([f32; 4], Self::VertexData) {
        let world_pos = self.model.to_homogeneous() * vertex.pos;
        let world_nor = self.model * vertex.nor;
        let light_dir = (self.light - world_pos.xyz()).normalize();
        let diffuse = world_nor.dot(&light_dir).clamp(0.0, 1.0).sqrt();
        let gouraud = vertex.col * (0.25 + 0.75 * diffuse);
        let clip_pos = self.proj * self.view * world_pos;

        (clip_pos.into(), Vertex { pos: clip_pos, col: gouraud, nor: world_nor })
    }

    #[inline(always)]
    fn fragment(&self, vertex: Self::VertexData) -> Self::Fragment {
        vertex.col
    }

    #[inline(always)]
    fn blend(&self, _: Self::Pixel, color: Self::Fragment) -> Self::Pixel {
        let r = (color.x * u8::MAX as f32).clamp(0.0, u8::MAX as f32) as u32;
        let g = (color.y * u8::MAX as f32).clamp(0.0, u8::MAX as f32) as u32;
        let b = (color.z * u8::MAX as f32).clamp(0.0, u8::MAX as f32) as u32;
        (r << 16) | (g << 8) | b
    }
}

/*
Physics utilities
*/

impl<const N: usize> Integrator<N> {
    fn step(&mut self) -> SVector<f32, N> {
        self.t += self.dt;
        self.state = self.rk4();
        self.state
    }

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

    fn rk4(&self) -> SVector<f32, N> {
        let k1 = (self.ddt)(&self.state);
        let k2 = (self.ddt)(&(self.state + k1 * (self.dt / 2.0)));
        let k3 = (self.ddt)(&(self.state + k2 * (self.dt / 2.0)));
        let k4 = (self.ddt)(&(self.state + k3 * self.dt));

        self.state + (k1 + k4) * self.dt / 6.0 + (k2 + k3) * self.dt / 3.0
    }
}

#[rustfmt::skip]
fn flippy_dymamics<const N: usize>(input: &SVector<f32, N>) -> SVector<f32, N> {
    let dcm = Matrix3::from_column_slice(input.rows(0, 9).as_slice());
    let ic = Matrix3::from_column_slice(input.rows(9, 9).as_slice());
    let omega = Vector3::from_column_slice(input.rows(18, 3).as_slice());
    let [i11, i22, i33] = [ic[(0, 0)], ic[(1, 1)], ic[(2, 2)]];
    let [w1, w2, w3] = [omega[0], omega[1], omega[2]];
    let skew = Matrix3::from_row_slice(&[
        0.0, -w3, w2 ,
        w3 , 0.0, -w1,
        -w2, w1 , 0.0,
    ]);

    let dcdt = -skew * dcm;
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

#[rustfmt::skip]
fn make_cuboid(
    lengths: Vector3<f32>,
    offset: Vector3<f32>,
    color: Vector4<f32>,
) -> (Vec<Vertex>, Vec<usize>) {
    type V = Vector3<f32>;
    let (len, off, col) = (lengths, offset, color);

    let nx = V::new(1.0, 0.0, 0.0);
    let ny = V::new(0.0, 1.0, 0.0);
    let nz = V::new(0.0, 0.0, 1.0);

    let positions = [
        V::new(-len.x, -len.y,  len.z), V::new( len.x, -len.y,  len.z), V::new( len.x,  len.y,  len.z),
        V::new(-len.x,  len.y,  len.z), V::new( len.x, -len.y, -len.z), V::new(-len.x, -len.y, -len.z),
        V::new(-len.x,  len.y, -len.z), V::new( len.x,  len.y, -len.z), V::new(-len.x, -len.y, -len.z),
        V::new(-len.x, -len.y,  len.z), V::new(-len.x,  len.y,  len.z), V::new(-len.x,  len.y, -len.z),
        V::new( len.x, -len.y,  len.z), V::new( len.x, -len.y, -len.z), V::new( len.x,  len.y, -len.z),
        V::new( len.x,  len.y,  len.z), V::new(-len.x,  len.y,  len.z), V::new( len.x,  len.y,  len.z),
        V::new( len.x,  len.y, -len.z), V::new(-len.x,  len.y, -len.z), V::new(-len.x, -len.y, -len.z),
        V::new( len.x, -len.y, -len.z), V::new( len.x, -len.y,  len.z), V::new(-len.x, -len.y,  len.z),
    ];

    let normals = [
         nz,  nz,  nz,  nz, -nz, -nz, -nz, -nz,
        -nx, -nx, -nx, -nx,  nx,  nx,  nx,  nx,
         ny,  ny,  ny,  ny, -ny, -ny, -ny, -ny,
    ];

    (
        positions
            .iter()
            .zip(normals.iter())
            .map(|(pos, nor)| Vertex { pos: (pos + off).push(1.0), col, nor: *nor })
            .collect(),
        (0..6)
            .flat_map(|idx| {
                let base = idx * 4;
                [base, base + 1, base + 2, base, base + 2, base + 3]
            })
            .collect(),
    )
}
