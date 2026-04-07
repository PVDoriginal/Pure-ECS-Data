use bevy::prelude::*;
use pure_ecs_data::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, PureDataPlugin));
    app.add_patch(osc_test);
    app.run();
}

patch!(
    osc_test;

    phasor = Phasor~ [1500];

    dac = Dac~;

    minus = Minus~ [1];
    multi = Mult~ [2];

    phasor => multi;
    multi => minus;

    minus => dac[0], dac[1];
);
