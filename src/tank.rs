use bevy::prelude::*;
use avian3d::prelude::*;


pub struct TankPlugin;
impl Plugin for TankPlugin{
  fn build(&self, app: &mut bevy::app::App) {
    app.add_systems(Startup, init_tank);
  }
}

fn init_tank(
  mut commands:Commands, 
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
){
  commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(3., 1.5, 6.),

        Mesh3d(meshes.add(Cuboid::new(3.,1.5,6.))),
        MeshMaterial3d(materials.add(Color::srgb_u8(0, 150, 50))),
        Transform::from_xyz(0.0, 2.0, 0.0),
    ));
}