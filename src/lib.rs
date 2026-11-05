mod bodies;
use bevy::{prelude::*};
use bodies::{Body, Sphere};

const GRAVITY: Vec3 = Vec3::new(0.0, -9.8, 0.0);

#[derive(Resource, Default)]
pub struct Scene {
    bodies: Vec<Body>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            bodies: vec![],
        }
    }

    pub fn initialize(&mut self) {
        let body = Body {
            position: Vec3::new(0.0, 1.0, 0.0),
            orientation: Quat::from_xyzw(0.0, 0.0, 0.0, 1.0),
            linear_velocity: Vec3::ZERO,
            inv_mass: 1.0,
            shape: Box::new(Sphere::new(1.0)),
        };
        self.bodies.push(body);
        let body = Body {
            position: Vec3::new(0.0, -1000.0, 0.0),
            orientation: Quat::from_xyzw(0.0, 0.0, 0.0, 1.0),
            linear_velocity: Vec3::ZERO,
            inv_mass: 0.0,
            shape: Box::new(Sphere::new(1000.0)),
        };
        self.bodies.push(body);
    }

    // Per physics frame update.
    pub fn update(&mut self, delta: f32) {
        for body in &mut self.bodies {
            let mass = 1.0 / body.inv_mass;
            let impulse = GRAVITY * mass * delta;
            body.apply_impulse_linear(&impulse);
        }

        for body in &mut self.bodies {
            body.position += body.linear_velocity;
        }
    }

    // Setup.
    pub fn setup(
        scene: ResMut<Scene>,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>) {
        commands.spawn((
            Camera3d::default(),
            Transform::from_xyz(5.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ));
        for (index, body) in scene.bodies.iter().enumerate() {
            body.setup(index, &mut commands, &mut meshes, &mut materials);
        }
    }

    // Sync body transforms to bevy.
    pub fn sync_bodies(
        scene: Res<Scene>,
        mut query: Query<(&bodies::BodyIndex, &mut Transform)>,
    ) {
        for (index, mut transform) in &mut query {
            let body = &scene.bodies[index.0];
            transform.translation = body.position;
            transform.rotation = body.orientation;
        }
    }
}
