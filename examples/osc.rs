use bevy::prelude::*;
use pure_ecs_data::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, PureDataPlugin));
    app.add_patch(counter);
    app.run();
}

patch!(
    counter;

    osc = Osc~ [400];
    dac = Dac~;

    osc => dac[0], dac[1];
);
