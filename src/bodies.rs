use std::hash::{DefaultHasher, Hash};

use bevy::prelude::*;
use rand::{SeedableRng, rngs::StdRng};

fn hash_to_f32(value: usize) -> f32 {
    let mut x = value as u64;

    x ^= x >> 30;
    x = x.wrapping_mul(0xbf58476d1ce4e5b9);
    x ^= x >> 27;
    x = x.wrapping_mul(0x94d049bb133111eb);
    x ^= x >> 31;

    // 取高 24 位，转换成 [0, 1)
    ((x >> 40) as f32) / 16_777_216.0
}

fn hash_to_color(index: usize) -> Color {
    Color::srgb(
        hash_to_f32(index.wrapping_mul(3)),
        hash_to_f32(index.wrapping_mul(3).wrapping_add(1)),
        hash_to_f32(index.wrapping_mul(3).wrapping_add(2)),
    )
}

pub enum ShapeType {
    Sphere,
}

pub trait Shape: Send + Sync  {
    fn get_type(&self) -> ShapeType;
    fn com(&self) -> Vec3;
    fn get_mesh(&self, meshes: &mut ResMut<Assets<Mesh>>) -> Mesh3d;
}

pub struct Sphere {
    radius: f32,
}

impl Sphere {
    pub fn new(radius: f32) -> Self {
        Sphere {
            radius
        }
    }
}

impl Shape for Sphere {
    fn get_type(&self) -> ShapeType {
        ShapeType::Sphere
    }
    fn com(&self) -> Vec3 {
        Vec3::ZERO
    }
    fn get_mesh(&self, meshes: &mut ResMut<Assets<Mesh>>) -> Mesh3d {
        Mesh3d(meshes.add(bevy::prelude::Sphere::new(self.radius)))
    }
}

pub struct Body {
    pub position: Vec3,
    pub orientation: Quat,
    pub linear_velocity: Vec3,
    pub inv_mass: f32,
    pub shape: Box<dyn Shape>,
}

#[derive(Component)]
pub struct BodyIndex(pub usize);

impl Body {
    pub fn com_to_worldspace(&self) -> Vec3 {
        let com = self.shape.com();
        self.position + self.orientation * com
    }

    pub fn worldspace_to_bodyspace(&self, pos: &Vec3) -> Vec3 {
        self.orientation.inverse() * pos - self.shape.com()
    }

    pub fn bodyspace_to_worldspace(&self, pos: &Vec3) -> Vec3 {
        self.position + self.orientation * (pos + self.shape.com())
    }

    pub fn apply_impulse_linear(&mut self, impulse: &Vec3) {
        if self.inv_mass == 0.0 {
            return;
        }
        self.linear_velocity += impulse * self.inv_mass;
    }

    pub fn setup(&self,
        index: usize,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>) {
        let mut hasher = DefaultHasher::new();
        commands.spawn((
            self.shape.get_mesh(meshes),
            MeshMaterial3d(
                materials.add(hash_to_color(index))
            ),
            Transform::from_xyz(self.position.x, self.position.y, self.position.z),
            BodyIndex(index),
        ));
    }
}

