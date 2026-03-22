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

    osc = Osc~ [500];
    dac = Dac~;

    mult = Mult~ [0.3];

    num1 = Number { 0.5 } # KeyA;
    num2 = Number { 1.5 } # KeyB;

    num1 -> mult;
    num2 -> mult;

    osc => mult;

    mult => dac[0], dac[1];
);
