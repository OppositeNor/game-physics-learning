use bevy::{DefaultPlugins, app::{FixedUpdate, Startup, Update}, ecs::system::{Res, ResMut}, time::Time};
use game_physics::*;

fn main() {
    let mut scene = Scene::new();
    scene.initialize();
    bevy::prelude::App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(scene)
        .add_systems(Startup, Scene::setup)
        .add_systems(FixedUpdate, |mut scene: ResMut<Scene>, time: Res<Time>| {
            scene.update(time.delta_secs());
        })
        .add_systems(Update, Scene::sync_bodies)
        .run();
}
