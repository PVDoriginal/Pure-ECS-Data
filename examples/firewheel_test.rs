//! A simple node that generates white noise.

use bevy::prelude::*;
use bevy_seedling::prelude::*;
use pure_ecs_data::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins,
            AssetPlugin::default(),
            SeedlingPlugin::default(),
        ))
        .register_node::<OscS>()
        .add_systems(
            Startup,
            |server: Res<AssetServer>, mut commands: Commands| {
                commands.spawn(OscS::default()).connect(MainBus);
            },
        )
        .run();
}
