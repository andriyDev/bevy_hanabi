use bevy::prelude::*;
use bevy_hanabi::prelude::*;

use crate::utils::AppExitIntoResult;

mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_exit = utils::DemoApp::new("from_asset")
        .build()
        .add_systems(Startup, setup)
        .run();
    app_exit.into_result()
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        Transform::from_xyz(3.0, 3.0, 5.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        Camera3d::default(),
    ));

    commands.spawn(ParticleEffect::new(asset_server.load("fire.effect")));
}
