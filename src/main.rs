mod map;
mod player;

use bevy::{
    prelude::*,
    window::{Window, WindowPlugin, WindowResolution},
};

use bevy_procedural_tilemaps::prelude::*;

use crate::map::generate::{MAP_PIXEL_DIMENSIONS, setup_generator};
use crate::player::PlayerPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(
                            MAP_PIXEL_DIMENSIONS.x as u32,
                            MAP_PIXEL_DIMENSIONS.y as u32,
                        ),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(ProcGenSimplePlugin::<Cartesian3D, Sprite>::default())
        .add_systems(Startup, (setup_camera, setup_generator))
        .add_plugins(PlayerPlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
