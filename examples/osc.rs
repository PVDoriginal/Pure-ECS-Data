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

    minus = Minus~ [0.5];
    multi = Mult~ [1000];

    phasor => minus;
    minus => multi;

    clip = Clip~;

    multi => clip;

    clip => dac[0], dac[1];
);
