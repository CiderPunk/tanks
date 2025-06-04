mod camera;
mod input;
mod scheduling;
mod state;
mod tank;

use avian3d::prelude::*;
use bevy::{asset::AssetMetaCheck, prelude::*};
use camera::CameraPlugin;
use input::GameInputPlugin;
use scheduling::SchedulingPlugin;
use state::StatePlugin;

use crate::tank::TankPlugin;


const APP_NAME: &str = "Tanks";

fn main() {
    App::new()
      .insert_resource(ClearColor(Color::srgb(0.1, 0.0, 0.15)))
      .insert_resource(AmbientLight {
        color: Color::default(),
        brightness: 750.0,
        ..Default::default()
      })
      .add_plugins(DefaultPlugins
        .set(WindowPlugin {
          primary_window: Some(Window {
            title: APP_NAME.into(),
            name: Some(APP_NAME.into()),
            fit_canvas_to_parent: true,
            visible: true,
            ..default()
          }),
          ..default()
        })
        .set(AssetPlugin {
          meta_check: AssetMetaCheck::Never,
          ..default()
        })
      )
      .add_plugins(PhysicsPlugins::default())
      .add_plugins((
        //CameraPlugin,
        GameInputPlugin,
        SchedulingPlugin,
        StatePlugin,
        TankPlugin,
      ))
      .add_systems(Startup, initialize)
      .run();
}

fn initialize(
    mut commands:Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
  ){

 // Static physics object with a collision shape
    commands.spawn((
        RigidBody::Static,
        Collider::cylinder(25.0, 0.1),
        Mesh3d(meshes.add(Cylinder::new(25.0, 0.1))),
        MeshMaterial3d(materials.add(Color::WHITE)),
    ));

/*
for i in (0..200){
commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(0.5, 0.5, 0.5),
        AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
        Mesh3d(meshes.add(Cuboid::from_length(0.5))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, i as f32 * 0.5, 0.0),
    ));
}
 */
    // Dynamic physics object with a collision shape and initial angular velocity
    

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-10., 6., 20.).looking_at(Vec3::ZERO, Dir3::Y),
    ));

}
