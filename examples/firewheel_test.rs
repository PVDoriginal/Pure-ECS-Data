//! A simple node that generates white noise.

use bevy::{input::keyboard::KeyboardInput, prelude::*};
use bevy_seedling::prelude::*;
use pure_ecs_data::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SeedlingPlugin::default()))
        .register_node::<OscS>()
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn(OscS { hertz: 300.0 }).connect(MainBus);
        })
        .add_systems(Update, update)
        .run();
}

fn update(
    osc: Single<Entity, With<OscS>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::KeyA) {
        commands
            .entity(*osc)
            .disconnect_with(MainBus, &[(0, 0), (0, 1)]);
        commands.entity(*osc).connect_with(MainBus, &[(0, 0)]);
    }
}
