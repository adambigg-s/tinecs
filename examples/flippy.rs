use euc::*;

use nalgebra::{Matrix, Matrix4, SMatrix, SVector, Vector4};

use tinecs::{
    Component, Master,
    arguments::{Query, With},
    master,
};

fn main() {
    let spinner = master().create_entity();
    let inertial = InertialTensor { inner: Matrix::default() };
    let velocity = AngularVelocity { inner: Matrix::default() };
    master().add_component(spinner, inertial);
    master().add_component(spinner, velocity);
    master().add_system(test_system);

    loop {
        master().run();
    }
}

struct Cube {
    mvp: Matrix4<f32>,
}

impl<'d> Pipeline<'d> for Cube {
    type Vertex = (Vector4<f32>, Vector4<f32>);

    type VertexData = Vector4<f32>;

    type Primitives = TriangleList;

    type Fragment = Vector4<f32>;

    type Pixel = Vector4<f32>;

    fn vertex(&self, vertex: &Self::Vertex) -> ([f32; 4], Self::VertexData) {
        todo!()
    }

    fn fragment(&self, vs_out: Self::VertexData) -> Self::Fragment {
        todo!()
    }

    fn blend(&self, old: Self::Pixel, new: Self::Fragment) -> Self::Pixel {
        todo!()
    }
}

fn test_system(query: Query<AngularVelocity, With<InertialTensor>>) {
    println!("wobbly spinner");
}

impl Component for RenderTarget {}
struct RenderTarget;

impl Component for InertialTensor {}
struct InertialTensor {
    inner: SMatrix<f32, 3, 3>,
}

impl Component for AngularVelocity {}
struct AngularVelocity {
    inner: SVector<f32, 3>,
}

impl Component for Rotation {}
struct Rotation {
    inner: SMatrix<f32, 3, 3>,
}
