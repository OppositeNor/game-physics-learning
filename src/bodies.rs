use bevy::prelude::*;

fn hash_to_f32(value: usize) -> f32 {
    let mut x = value as u64;

    x ^= x >> 30;
    x = x.wrapping_mul(0xbf58476d1ce4e5b9);
    x ^= x >> 27;
    x = x.wrapping_mul(0x94d049bb133111eb);
    x ^= x >> 31;
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
    Sphere {radius: f32},
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
        ShapeType::Sphere { radius: self.radius }
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

#[derive(Default)]
pub struct Contact<'a> {
    pub pt_on_a_ws: Vec3,
    pub pt_on_a_ls: Vec3,
    pub pt_on_b_ws: Vec3,
    pub pt_on_b_ls: Vec3,

    pub normal: Vec3,
    pub separation_disntance: f32,
    pub time_of_impact: f32,

    pub a: Option<&'a mut Body>,
    pub b: Option<&'a mut Body>,
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

pub fn intersect<'a>(body_a: &'a mut Body, body_b: &'a mut Body, contact: &mut Contact<'a>) -> bool {
    match (body_a.shape.get_type(), body_b.shape.get_type()) {
        (ShapeType::Sphere { radius: r_a }, ShapeType::Sphere { radius: r_b }) => {
            let (pos_a, pos_b) = (body_a.position, body_b.position);
            contact.a = Some(body_a);
            contact.b = Some(body_b);
            let a_to_b = pos_b - pos_a;

            contact.normal = a_to_b.normalize();
            contact.pt_on_a_ws = pos_a + contact.normal * r_a;
            contact.pt_on_b_ws = pos_b - contact.normal * r_b;

            let length_squared = (a_to_b).length_squared();
            let r_sum = r_a + r_b;
            length_squared <= r_sum * r_sum
        }
    }
}

pub fn resolve_contact(contact: &mut Contact) {
    if let (Some(a), Some(b)) = (&mut contact.a, &mut contact.b ){
        a.linear_velocity = Vec3::ZERO;
        b.linear_velocity = Vec3::ZERO;

        let denom = 1.0f32 / (a.inv_mass + b.inv_mass);
        let t_a = a.inv_mass / denom;
        let t_b = b.inv_mass / denom;

        let delta = contact.pt_on_b_ws - contact.pt_on_a_ws;
        a.position += delta * t_a;
        b.position -= delta * t_b;
    }
}
